use tokio::net::{TcpListener,TcpStream};
use tokio::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use scrap::{Capturer, Display};
use std::thread::sleep;
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

// use crate::app::helper::get_local_ip;
use crate::app::auth;


async fn capture_and_stream(mut stream: TcpStream) -> Result<(), IoError> {
    let desktop_window: HWND = unsafe { GetDesktopWindow() };
    let desktop_dc: HDC = unsafe { GetWindowDC(desktop_window) };
    let compatible_dc: HDC = unsafe { CreateCompatibleDC(desktop_dc) };

    let width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    let height = unsafe { GetSystemMetrics(SM_CYSCREEN) };

    let bitmap: HBITMAP = unsafe { CreateCompatibleBitmap(desktop_dc, width, height) };
    unsafe { SelectObject(compatible_dc, bitmap as _) };

    loop {
        // Capture the screen
        unsafe {
            BitBlt(
                compatible_dc,
                0,
                0,
                width,
                height,
                desktop_dc,
                0,
                0,
                SRCCOPY,
            );
        }

        // Convert to ImageBuffer
        let mut buffer: Vec<u8> = vec![0; (width * height * 4) as usize];
        unsafe {
            winapi::um::wingdi::GetBitmapBits(
                bitmap,
                (width * height * 4) as i32,
                buffer.as_mut_ptr() as _,
            );
        }

        let image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(width as u32, height as u32, buffer)
            .expect("Failed to create ImageBuffer");

        // Compress the image data
        let compressed = compress_prepend_size(&image);

        // Send the compressed data
        stream.write_all(&compressed).await?;

        // Add a small delay to control the frame rate
        tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;
    }

    // Clean up (this part is never reached in the infinite loop, but included for completeness)
    unsafe {
        DeleteObject(bitmap as _);
        DeleteDC(compatible_dc);
    }

    Ok(())
}

async fn run_remote_desktop_server(_token: HANDLE, mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {

        // let one_frame_duration = Duration::from_millis(100);
        // let display = Display::primary().expect("Couldn't find the primary display.");
        // let mut capturer = Capturer::new(display).expect("Couldn't begin capturing.");

        // loop {
        //     match capturer.frame() {
        //         Ok(frame) => {
        //             // Send the frame to the client
        //             if let Err(_) = socket.write_all(&frame).await {
        //                 println!("Client disconnected.");
        //                 break;
        //             }
        //             // Slow down the loop to control frame rate
        //             sleep(one_frame_duration);
        //         }
        //         Err(_) => {
        //             // If the screen is being switched or other error, retry
        //             // capturer = Capturer::new(display).expect("Couldn't reinitialize capturing.");
        //         }
        //     }
        // }

        capture_and_stream(socket).await?;
    Ok(())
  
}

pub async fn server() -> Result<(), Box<dyn std::error::Error>>  {

    // let address = get_local_ip().await?;
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Connection received from: {}", addr);

        // Read credentials from client
        let mut buffer = [0; 1024];
        let n = socket.read(&mut buffer).await.unwrap();
        let received_data = String::from_utf8_lossy(&buffer[..n]);
        let credentials: auth::Credentials = serde_json::from_str(&received_data).unwrap();
        println!("Received credentials: {:?}", credentials);

        match auth::authenticate_user(credentials) {
            Ok(token) => {
                let error_message = "Authentication Successful";
                socket.write_all(error_message.as_bytes()).await?;
                socket.flush().await?;

                tokio::spawn(async move {
                    if let Err(e) = run_remote_desktop_server(token, socket).await {
                        eprintint!("Error in start function: {}", e);
                    }
                });
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