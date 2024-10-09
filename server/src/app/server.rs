use tokio::net::{TcpListener,TcpStream};
use tokio::io::{AsyncReadExt, ReadHalf, WriteHalf,AsyncWriteExt};
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::HANDLE;
use std::io::Error as IoError;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::app::auth;
use crate::app::read_inputs;
use crate::app::stream;


async fn run_remote_desktop_server(socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {

    let (mut read_half, mut write_half) = tokio::io::split(socket);
    stream::capture_and_stream(write_half).await?;
    read_inputs::read_user_input_make_changes(read_half).await?;
    Ok(())
}

pub async fn server() -> Result<(), Box<dyn std::error::Error>>  {

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server listening on 0.0.0.0:3000");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Connection received from: {}", addr);

        let mut buffer = [0; 1024];
        let n = socket.read(&mut buffer).await.unwrap();
        let connection_settings = String::from_utf8_lossy(&buffer[..n]);

        match auth::authenticate_user(connection_settings.to_string()).await {
            Ok(token) => {
                let message: bool = true;
                let byte_message: u8 = if message { 1 } else { 0 };
                socket.write_all(&[byte_message]).await?;
                socket.flush().await?;

                // tokio::spawn(async move {
                if let Err(e) = run_remote_desktop_server(socket).await {
                    eprintln!("Error in start function: {}", e);
                }
                // });
                unsafe { CloseHandle(token) };
            },
            Err(e) => {
                let message: bool = false;
                let byte_message: u8 = if message { 1 } else { 0 };
                socket.write_all(&[byte_message]).await?;
                socket.flush().await?;
            }
        }  

    }
}