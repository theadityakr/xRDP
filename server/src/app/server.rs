use futures::TryFutureExt;
use tokio::net::{TcpListener,TcpStream};
use tokio::io::{AsyncReadExt, ReadHalf, WriteHalf,AsyncWriteExt};
use std::net::SocketAddr;
use std::sync::Arc;
use std::error::Error;
use tokio::sync::Mutex;
use std::collections::HashMap;
use windows::Win32::System::RemoteDesktop::{
    WTSOpenServerA, WTSCloseServer, WTSQuerySessionInformationA,
    WTSDisconnectSession, WTSLogoffSession, WTS_CURRENT_SESSION,
    WTSUserName, WTSClientProtocolType
};
use windows::Win32::Foundation::{HANDLE, CloseHandle};
use windows::core::{PWSTR, PSTR, PCSTR};
use rand::Rng;
use tokio::task::LocalSet;

use crate::app::auth;
use crate::app::stream_handler;
use crate::app::input_handler;
// use crate::app::wts;
// use crate::app::drive_protocol;


struct Session {
    token: HANDLE,
    addr: SocketAddr,
    username : String,
}

async fn run_remote_desktop_server(session_id: u32, stream: TcpStream, sessions: Arc<Mutex<HashMap<u32, Session>>>,) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (mut read_half, mut write_half) = tokio::io::split(stream);

    tokio::spawn(async move {
        if let Err(e) = stream_handler::capture_and_stream(write_half).await {
            eprintln!("[capture_and_stream] [Error] [{}]: {}", &session_id, e);
        }
    });

    tokio::spawn(async move {
        if let Err(e) = input_handler::read_and_apply_input(read_half).await {
            eprintln!("[read_and_apply_input] [Error] [{}]: {}", &session_id, e);
        }
    });

    Ok(())
}

async fn handle_client(addr: SocketAddr, mut stream: TcpStream, sessions: Arc<Mutex<HashMap<u32, Session>>>,) -> Result<(), Box<dyn Error + Send + Sync>>{
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    let credentials = String::from_utf8_lossy(&buffer[..n]);

    match auth::authenticate_user(credentials.to_string()).await {
        Ok((token, username)) => {
            println!("[handle_client] [Debug] [token]: {:?}",&token);
            let session_id = create_or_get_session(token).await?;

            println!("[handle_client]->[create_or_get_session] [Debug] [session_id]: {:?}",&session_id);
            {
                let mut sessions_lock = sessions.lock().await;
                sessions_lock.insert(session_id, Session { token, addr, username });
              
            }

            if let Err(e) = stream.write_all(&[1]).await {
                eprintln!("[handle_client] [Error] Failed to write auth success: {}", e);
                return Err(Box::new(e));
            }

            if let Err(e) = stream.write_all(&session_id.to_le_bytes()).await {
                eprintln!("[handle_client] [Error] Failed to write session_id: {}", e);
                return Err(Box::new(e)); 
            }

            if let Err(e) = stream.flush().await {
                eprintln!("[handle_client] [Error] Failed to flush stream: {}", e);
                return Err(Box::new(e)); 
            }
            
            run_remote_desktop_server(session_id, stream, sessions).await;
        },
        Err(_) => {
            if let Err(e) = stream.write_all(&[0]).await {
                eprintln!("[handle_client] [Error] Failed to write auth failure: {}", e);
            }
            if let Err(e) = stream.flush().await {
                eprintln!("[handle_client] [Error] Failed to flush auth failure: {}", e);
            }
        }
    }
    Ok(())
}

async fn create_or_get_session(token: HANDLE) -> Result<u32, Box<dyn Error + Send + Sync>>{
    unsafe {
        let server_handle = WTSOpenServerA(PCSTR::null());
        if server_handle.is_invalid() {
            return Err("[create_or_get_session] [Error] : Failed to open server".into());
        }

        let mut bytes_returned: u32 = 0;
        let mut buffer: PWSTR = PWSTR::null();
        
        let result = WTSQuerySessionInformationA(
            server_handle,
            WTS_CURRENT_SESSION,
            WTSClientProtocolType,
            &mut buffer as *mut PWSTR as *mut PSTR,
            &mut bytes_returned,
        );

        if !result.as_bool() {
            WTSCloseServer(server_handle);
            return Err("[create_or_get_session] [Error] : Failed to query session information".into());
        }

        let mut rng = rand::thread_rng();
        let session_id =  rng.gen::<u32>();

        WTSCloseServer(server_handle);

        Ok(session_id)
    }
}

async fn cleanup_session(token: HANDLE, session_id: u32) {
    unsafe {
        let server_handle = WTSOpenServerA(PCSTR::null());
        if !server_handle.is_invalid() {
            WTSDisconnectSession(server_handle, session_id, false);
            WTSLogoffSession(server_handle, session_id, false);
            WTSCloseServer(server_handle);
        }
        CloseHandle(token);
    }
}

pub async fn server() -> Result<(), Box<dyn Error + Send + Sync>> {
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("[server] [Debug] : Server listening on 0.0.0.0:3000");

    let sessions = Arc::new(Mutex::new(HashMap::new()));

    while let Ok((stream, addr)) = listener.accept().await {
        let sessions = Arc::clone(&sessions);
        handle_client(addr, stream, sessions).await;
        // tokio::spawn(async move {
        //     handle_client(addr, stream, sessions).await;
        // });
    }

    Ok(())
}
