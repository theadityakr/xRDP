use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use image::{ImageBuffer, Rgb};
use minifb::{Window, WindowOptions, Key};
use std::error::Error;
use std::sync::Arc;

pub async fn render_screen(reader: Arc<Mutex<BufReader<TcpStream>>>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut window = Window::new(
        "Remote Desktop Client",
        1920,
        1080,
        WindowOptions::default(),
    )?;

    let mut buffer: Vec<u32> = vec![0; 1920 * 1080];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Lock the reader to use it
        let mut reader = reader.lock().await;

        // Read the size of the incoming data
        let mut size_bytes = [0u8; 4];
        reader.read_exact(&mut size_bytes).await?;
        let size = u32::from_be_bytes(size_bytes) as usize;

        // Read the screen data
        let mut screen_data = vec![0u8; size];
        reader.read_exact(&mut screen_data).await?;

        // Release the lock
        drop(reader);

        // Convert raw RGB data to ImageBuffer
        let img = ImageBuffer::<Rgb<u8>, _>::from_raw(1920, 1080, screen_data)
            .ok_or("Failed to create ImageBuffer")?;

        // Convert ImageBuffer to minifb compatible format
        for (i, pixel) in img.pixels().enumerate() {
            let [r, g, b] = pixel.0;
            buffer[i] = (r as u32) << 16 | (g as u32) << 8 | (b as u32);
        }

        // Update the window with new data
        window.update_with_buffer(&buffer, 1920, 1080)?;
    }

    Ok(())
}

