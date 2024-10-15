use futures::TryFutureExt;
use tokio::net::{TcpListener,TcpStream};
use tokio::io::{AsyncReadExt, ReadHalf, WriteHalf,AsyncWriteExt};
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
use crate::app::read_inputs;
use crate::app::stream;
// use crate::app::drive_protocol;


struct Session {
    token: HANDLE,
    session_id: u32,
}

async fn run_remote_desktop_server(socket: TcpStream,addr: std::net::SocketAddr, username:String,  session_id: u32,) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (mut read_half, mut write_half) = tokio::io::split(socket);

    let local = LocalSet::new();

    // Capture and stream screen data
    local.spawn_local(async move {
        if let Err(e) = stream::capture_and_stream(write_half).await {
            eprintln!("Error handling client {}: {}", addr, e);
        }
    });

    // Read user inputs and make changes
    local.spawn_local(async move {
        if let Err(e) = read_inputs::read_user_input_make_changes(read_half).await {
            eprintln!("Error handling client {}: {}", addr, e);
        }
    });

    local.await;

    Ok(())
}

async fn handle_client(mut socket: TcpStream, addr: std::net::SocketAddr, sessions: Arc<Mutex<HashMap<String, Session>>>,) -> Result<(), Box<dyn Error + Send + Sync>>{
    let mut buffer = [0; 1024];
    let n = socket.read(&mut buffer).await?;
    let credentials = String::from_utf8_lossy(&buffer[..n]);

    match auth::authenticate_user(credentials.to_string()).await {
        Ok((token, username)) => {
            println!("{:?}",&token);
            let session_id = create_or_get_session(token).await?;

            println!("{:?}",&session_id);
            let mut sessions = sessions.lock().await;
            sessions.insert(username.clone(), Session { token, session_id });

            socket.write_all(&[1]).await?; 
            socket.flush().await?;

            run_remote_desktop_server(socket, addr, username, session_id).await;
        },
        Err(_) => {
            socket.write_all(&[0]).await?; 
            socket.flush().await?;
        }
    }
    Ok(())
}

async fn create_or_get_session(token: HANDLE) -> Result<u32, Box<dyn Error + Send + Sync>>{
    unsafe {
        let server_handle = WTSOpenServerA(PCSTR::null());
        if server_handle.is_invalid() {
            return Err("Failed to open server".into());
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
            return Err("Failed to query session information".into());
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
    println!("Server listening on 0.0.0.0:3000");

    let sessions = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Connection received from: {}", addr);

        let sessions = Arc::clone(&sessions);
        handle_client(socket, addr, sessions).await?;
        // tokio::spawn(async move {
        //     if let Err(e) = handle_client(socket, addr, sessions).await {
        //         eprintln!("Error handling client {}: {}", addr, e);
        //     }
        // });
    }
}
