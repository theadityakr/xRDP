use tokio::net::{TcpListener,TcpStream};
use tokio::io::{AsyncReadExt, ReadHalf, WriteHalf,AsyncWriteExt};
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::HANDLE;
use std::io::Error as IoError;
use winapi::um::winuser::{GetDesktopWindow, GetWindowDC, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use winapi::um::wingdi::{BitBlt, CreateCompatibleDC, CreateCompatibleBitmap, SelectObject, SRCCOPY};
use winapi::shared::windef::{HWND, HDC, HBITMAP};
use winapi::um::wingdi::DeleteDC;
use winapi::um::wingdi::DeleteObject;
use image::{ImageBuffer, Rgba};
use lz4_flex::block::compress_prepend_size;
use std::sync::Arc;
use tokio::sync::Mutex;

// use crate::app::helper::get_local_ip;
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
                // let message = "Authentication Successful";
                // socket.write_all(message.as_bytes()).await?;
                // socket.flush().await?;

                // tokio::spawn(async move {
                if let Err(e) = run_remote_desktop_server(socket).await {
                    eprintln!("Error in start function: {}", e);
                }
                // });
                unsafe { CloseHandle(token) };
            },
            Err(e) => {
                println!("Authentication failed: {}", e);
                let error_message = "Authentication failed: Incorrect Username / password";
                socket.write_all(error_message.as_bytes()).await?;
                socket.flush().await?;
            }
        }  

    }
}