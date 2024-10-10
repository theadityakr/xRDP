use lz4_flex::compress_prepend_size;
use tokio::io::AsyncWriteExt;
use image::{ImageBuffer, Rgba};
use winapi::shared::windef::{HDC, HWND};
use winapi::um::wingdi::{BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetBitmapBits, SelectObject, SRCCOPY};
use winapi::um::winuser::{GetSystemMetrics, GetDesktopWindow, GetWindowDC, SM_CXSCREEN, SM_CYSCREEN};
use tokio::io::WriteHalf;
use tokio::net::TcpStream;
use std::io::Error as IoError;

pub async fn capture_and_stream(mut stream: WriteHalf<TcpStream>) -> Result<(), IoError> {
    let desktop_window: HWND = unsafe { GetDesktopWindow() };
    let desktop_dc: HDC = unsafe { GetWindowDC(desktop_window) };
    let compatible_dc: HDC = unsafe { CreateCompatibleDC(desktop_dc) };

    let width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    let height = unsafe { GetSystemMetrics(SM_CYSCREEN) };
    println!("Screen dimensions: {}x{}", width, height);

    let bitmap = unsafe { CreateCompatibleBitmap(desktop_dc, width, height) };
    unsafe { SelectObject(compatible_dc, bitmap as _) };

    loop {
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

        // Step 1: Compress the image data
        let compressed = compress_prepend_size(&image);

        // Step 2: Send the size of the compressed data first (in a fixed-size header)
        let compressed_size = compressed.len() as u64;
        let size_buffer = compressed_size.to_le_bytes(); // Sending the size in little-endian format
        stream.write_all(&size_buffer).await?;

        // Step 3: Send the actual compressed data
        stream.write_all(&compressed).await?;

        // Sleep for a short duration before the next capture
        tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;
    }

    // Cleanup resources
    unsafe {
        DeleteObject(bitmap as _);
        DeleteDC(compatible_dc);
    }

    Ok(())
}
