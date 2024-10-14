use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, ReadHalf, AsyncWriteExt, WriteHalf};
use std::error::Error;
use std::sync::mpsc;
use std::thread;
use minifb::{Window, WindowOptions, Key};
use lz4_flex::decompress_size_prepended;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const MAX_BUFFER_SIZE: usize = WIDTH * HEIGHT * 4;

pub async fn render_screen(mut stream: ReadHalf<TcpStream>,mut write_half:WriteHalf<TcpStream>) -> Result<(), Box<dyn Error + Send + Sync>> {
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
        // Step 1: Read the compressed data size from the stream
        let mut size_buffer = [0u8; 8];
        stream.read_exact(&mut size_buffer).await?;
        let compressed_size = u64::from_le_bytes(size_buffer) as usize;
    
        println!("Received compressed size: {}", compressed_size);
        
        // Step 2: Read the compressed data from the stream
        let mut compressed_data = vec![0u8; compressed_size];
        stream.read_exact(&mut compressed_data).await?;
    
        println!("Received compressed data: {:?}", &compressed_data[..10]);
        
        // Step 3: Decompress the data
        let decompressed_data = decompress_size_prepended(&compressed_data)?;
    
        println!("Decompressed data length: {}", decompressed_data.len());

        // Step 4: Check if the decompressed data has a valid size
        if decompressed_data.len() != MAX_BUFFER_SIZE {
            return Err(format!("Invalid image size: {} bytes", decompressed_data.len()).into());
        }

        // Step 5: Send the decompressed data to the rendering thread
        tx.send(decompressed_data).map_err(|e| format!("Failed to send buffer: {}", e))?;
    }
}
