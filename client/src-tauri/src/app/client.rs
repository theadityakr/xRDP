use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use serde::{Serialize, Deserialize};
use serde_json::Result as JsonResult;
use tokio::task;
use tokio::io::{ReadHalf, WriteHalf};
use std::error::Error;
use std::sync::{Arc, Mutex};
use image::{ImageBuffer, Rgb};
use minifb::{Window, WindowOptions, Key};
use std::time::Duration;

use crate::app::render;
use crate::app::input;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ConnectionSettings {
    computer: String,
    username: String,
    password: String,
    general_settings: GeneralSettings,
    advanced_settings: AdvancedSettings,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GeneralSettings {
    save_password: bool,
    multiple_display: bool,
    local_drives_redirection: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AdvancedSettings {
    printers: bool,
    clipboard: bool,
}

#[derive(Debug)]
struct Credentials {
    address: String,
    username: String,
    password: String,
}

async fn initial_check(connection_settings: String) -> JsonResult<Credentials> {
    let settings: ConnectionSettings = serde_json::from_str(&connection_settings)?;
    
    Ok(Credentials {
        address: settings.computer,
        username: settings.username,
        password: settings.password
    })
}

pub async fn start_client(connection_settings: &str) -> Result<(),Box<dyn Error + Send + Sync>> {

    let credentials = initial_check(connection_settings.to_string()).await?;
    let mut stream = TcpStream::connect(&credentials.address).await?;
    println!("Connected to the server at {}", &credentials.address);
    stream.write_all(connection_settings.as_bytes()).await?;

    let reader = BufReader::new(stream);
    let reader = Arc::new(tokio::sync::Mutex::new(reader));

    // Create a channel to send screen data from the network thread to the render thread
    let (tx, rx) = std::sync::mpsc::channel();

    // Spawn a tokio task to handle network operations
    let network_handle = tokio::spawn(async move {
        loop {
            if let Err(e) = async {
                let mut reader = reader.lock().await;
                
                // Read the size of the incoming data
                let mut size_bytes = [0u8; 4];
                reader.read_exact(&mut size_bytes).await?;
                let size = u32::from_be_bytes(size_bytes) as usize;

                // Read the screen data
                let mut screen_data = vec![0u8; size];
                reader.read_exact(&mut screen_data).await?;

                // Send the screen data to the render thread
                tx.send(screen_data).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

                Ok::<_, std::io::Error>(())
            }.await {
                eprintln!("Error in network thread: {}", e);
                break;
            }
        }
    });

    // Create the window in the main thread
    let mut window = Window::new(
        "Remote Desktop Client",
        1920,
        1080,
        WindowOptions::default(),
    )?;

    let mut buffer: Vec<u32> = vec![0; 1920 * 1080];

    // Main render loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Try to receive new screen data
        if let Ok(screen_data) = rx.try_recv() {
            // Convert raw RGB data to ImageBuffer
            if let Some(img) = ImageBuffer::<Rgb<u8>, _>::from_raw(1920, 1080, screen_data) {
                // Convert ImageBuffer to minifb compatible format
                for (i, pixel) in img.pixels().enumerate() {
                    let [r, g, b] = pixel.0;
                    buffer[i] = (r as u32) << 16 | (g as u32) << 8 | (b as u32);
                }
            }
        }

        // Update the window with new data
        window.update_with_buffer(&buffer, 1920, 1080)?;

        // Sleep to control frame rate
        std::thread::sleep(Duration::from_millis(16)); // ~60 fps
    }

    // If we've exited the render loop, cancel the network task
    network_handle.abort();

    Ok(())
}