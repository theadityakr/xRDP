use lz4_flex::compress_prepend_size;
use tokio::io::AsyncWriteExt;
use image::{ImageBuffer, Rgba};
use winapi::shared::windef::{HDC, HWND};
use winapi::um::wingdi::{BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetBitmapBits, SelectObject, SRCCOPY};
use winapi::um::winuser::{GetSystemMetrics, GetDesktopWindow, GetWindowDC, SM_CXSCREEN, SM_CYSCREEN};
use tokio::io::WriteHalf;
use tokio::net::TcpStream;
use std::error::Error;


use crate::app::input_handler::get_screen_resolution;

fn capture_screen(width: i32, height: i32) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
    let desktop_window: HWND = unsafe { GetDesktopWindow() };
    let desktop_dc: HDC = unsafe { GetWindowDC(desktop_window) };
    let compatible_dc: HDC = unsafe { CreateCompatibleDC(desktop_dc) };

    let bitmap = unsafe { CreateCompatibleBitmap(desktop_dc, width, height) };
    unsafe { SelectObject(compatible_dc, bitmap as _) };

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

    let mut buffer: Vec<u8> = vec![0; (width * height * 4) as usize];
    unsafe {
        GetBitmapBits(
            bitmap,
            (width * height * 4) as i32,
            buffer.as_mut_ptr() as _,
        );
    }

    let image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(width as u32, height as u32, buffer)
        .expect("Failed to create ImageBuffer");

    // Compress the image data
    let compressed = compress_prepend_size(&image);

    // Cleanup resources
    unsafe {
        DeleteObject(bitmap as _);
        DeleteDC(compatible_dc);
    }

    Ok(compressed)
}

pub async fn capture_and_stream(mut stream: WriteHalf<TcpStream>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (screen_width, screen_height) = get_screen_resolution();

    loop {
        // Capture and compress the screen in a synchronous function
        let compressed = capture_screen(screen_width, screen_height)?;

        // Send the size of the compressed data first (in a fixed-size header)
        let compressed_size = compressed.len() as u64;
        let size_buffer = compressed_size.to_le_bytes(); // Sending the size in little-endian format
        stream.write_all(&size_buffer).await?;

        // Send the actual compressed data
        stream.write_all(&compressed).await?;

        // Sleep for a short duration before the next capture
        tokio::time::sleep(tokio::time::Duration::from_millis(33)).await;
    }
}
