use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, ReadHalf};
use std::error::Error;
use std::sync::mpsc;
use std::thread;
use minifb::{Window, WindowOptions, Key};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const MAX_BUFFER_SIZE: usize = WIDTH * HEIGHT * 4;

pub async fn render_screen(mut stream: ReadHalf<TcpStream>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (tx, rx) = mpsc::channel::<Vec<u8>>();

    thread::spawn(move || {
        let mut window = Window::new(
            "Remote Desktop",
            WIDTH,
            HEIGHT,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("Failed to create window: {}", e);
        });

        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

        while window.is_open() && !window.is_key_down(Key::Escape) {
            if let Ok(image_buffer) = rx.try_recv() {
                println!("Received buffer of size: {}", image_buffer.len());
                
                for (i, chunk) in image_buffer.chunks_exact(4).enumerate() {
                    if i < buffer.len() {
                        if let [b, g, r, _] = chunk {
                            // Directly use RGB values without any conversion
                            buffer[i] = u32::from_le_bytes([*b, *g, *r, 255]);
                        }
                    } else {
                        break;
                    }
                }
                
                println!("First pixel value: {:x}", buffer[0]);
            }

            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        }
    });

    loop {
        let mut size_buffer = [0u8; 8];
        stream.read_exact(&mut size_buffer).await?;
        let image_size = u64::from_le_bytes(size_buffer) as usize;

        println!("Received image size: {}", image_size);

        if image_size > MAX_BUFFER_SIZE || image_size == 0 {
            return Err(format!("Invalid image size: {} bytes", image_size).into());
        }

        let mut buffer = vec![0u8; image_size];
        stream.read_exact(&mut buffer).await?;

        println!("Read buffer of size: {}", buffer.len());
        if !buffer.is_empty() {
            println!("First byte: {:x}", buffer[0]);
        }

        tx.send(buffer).map_err(|e| format!("Failed to send buffer: {}", e))?;
    }
}