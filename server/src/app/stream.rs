// use tokio::net::{TcpListener,TcpStream};
// use tokio::io::{AsyncReadExt, ReadHalf, WriteHalf,AsyncWriteExt};
// use winapi::um::handleapi::CloseHandle;
// use winapi::um::winnt::HANDLE;
// use std::io::Error as IoError;
// use winapi::um::winuser::{GetDesktopWindow, GetWindowDC, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
// use winapi::um::wingdi::{BitBlt, CreateCompatibleDC, CreateCompatibleBitmap, SelectObject, SRCCOPY};
// use winapi::shared::windef::{HWND, HDC, HBITMAP};
// use winapi::um::wingdi::DeleteDC;
// use winapi::um::wingdi::DeleteObject;
// use image::{ImageBuffer, Rgba};
// use lz4_flex::block::compress_prepend_size;
// use std::sync::Arc;
// use tokio::sync::Mutex;


// pub async fn capture_and_stream(mut stream: WriteHalf<TcpStream>) -> Result<(), IoError> {
//     let desktop_window: HWND = unsafe { GetDesktopWindow() };
//     let desktop_dc: HDC = unsafe { GetWindowDC(desktop_window) };
//     let compatible_dc: HDC = unsafe { CreateCompatibleDC(desktop_dc) };

//     let width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
//     let height = unsafe { GetSystemMetrics(SM_CYSCREEN) };

//     let bitmap: HBITMAP = unsafe { CreateCompatibleBitmap(desktop_dc, width, height) };
//     unsafe { SelectObject(compatible_dc, bitmap as _) };

//     loop {
//         // Capture the screen
//         unsafe {
//             BitBlt(
//                 compatible_dc,
//                 0,
//                 0,
//                 width,
//                 height,
//                 desktop_dc,
//                 0,
//                 0,
//                 SRCCOPY,
//             );
//         }

//         // Convert to ImageBuffer
//         let mut buffer: Vec<u8> = vec![0; (width * height * 4) as usize];
//         unsafe {
//             winapi::um::wingdi::GetBitmapBits(
//                 bitmap,
//                 (width * height * 4) as i32,
//                 buffer.as_mut_ptr() as _,
//             );
//         }

//         let image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(width as u32, height as u32, buffer)
//             .expect("Failed to create ImageBuffer");

//         // Compress the image data
//         let compressed = compress_prepend_size(&image);

//         // Send the compressed data
//         stream.write_all(&compressed).await?;

//         // Add a small delay to control the frame rate
//         tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;
//     }

//     // Clean up (this part is never reached in the infinite loop, but included for completeness)
//     unsafe {
//         DeleteObject(bitmap as _);
//         DeleteDC(compatible_dc);
//     }

//     Ok(())
// }

use tokio::io::{AsyncWriteExt, AsyncReadExt, WriteHalf};
use tokio::net::TcpStream;
use std::error::Error;


fn capture_screen() -> Vec<u8> {
    // Mock implementation of screen capture
    let mut screen_data = Vec::with_capacity(1920 * 1080 * 3);
    for y in 0..1080 {
        for x in 0..1920 {
            // Create a gradient pattern
            let r = (x as f32 / 1920.0 * 255.0) as u8;
            let g = (y as f32 / 1080.0 * 255.0) as u8;
            let b = ((x + y) as f32 / 3000.0 * 255.0) as u8;
            screen_data.extend_from_slice(&[r, g, b]);
        }
    }
    screen_data
}

pub async fn capture_and_stream(mut write_half: WriteHalf<TcpStream>) -> Result<(), Box<dyn Error>> {
    println!("Starting capture_and_stream");
    let mut frame_count = 0;
    loop {
        // Capture screen data
        let screen_data = capture_screen();

        // Send the size of the data first
        let size = screen_data.len() as u32;
        write_half.write_all(&size.to_be_bytes()).await?;
        println!("Sent size: {}", size);

        // Send the screen data
        write_half.write_all(&screen_data).await?;
        println!("Sent frame Number: {}", frame_count);

        frame_count += 1;
        
        // Control the frame rate (1 fps for debugging)
        tokio::time::sleep(tokio::time::Duration::from_millis(60)).await;
    }
}


