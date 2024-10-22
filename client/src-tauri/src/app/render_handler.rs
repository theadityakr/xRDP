use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, ReadHalf, AsyncWriteExt, WriteHalf};
use std::error::Error;
use std::sync::mpsc;
use std::thread;
use minifb::{Window, WindowOptions, Key};
use lz4_flex::decompress_size_prepended;

#[cfg(target_os = "windows")]
fn get_screen_resolution() -> (usize, usize) {
    use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

    let width = unsafe { GetSystemMetrics(SM_CXSCREEN) } as usize;
    let height = unsafe { GetSystemMetrics(SM_CYSCREEN) } as usize;

    (width, height)
}

#[cfg(target_os = "macos")]
fn get_screen_resolution() -> (usize, usize) {
    use core_graphics::display::CGDisplay;

    let display = CGDisplay::main();
    let width = display.pixels_wide() as usize;
    let height = display.pixels_high() as usize;

    (width, height)
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn get_screen_resolution() -> (usize, usize) {
    (1920, 1080) 
}

pub async fn render_screen(mut stream: ReadHalf<TcpStream>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (tx, rx) = mpsc::channel::<Vec<u8>>();
    let (screen_width, screen_height) = get_screen_resolution();
    let max_buffer_size: usize = screen_width * screen_height * 4;
    

    thread::spawn(move || {
        let mut window = Window::new(
            "Remote Desktop Client",
            screen_width,
            screen_height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("Failed to create window: {}", e);
        });

        let mut buffer: Vec<u32> = vec![0; screen_width * screen_height];

        while window.is_open() && !window.is_key_down(Key::Escape) {
            if let Ok(image_buffer) = rx.try_recv() {
                for (i, chunk) in image_buffer.chunks_exact(4).enumerate() {
                    if i < buffer.len() {
                        if let [b, g, r, _] = chunk {
                            buffer[i] = u32::from_le_bytes([*b, *g, *r, 255]);
                        }
                    } else {
                        break;
                    }
                }
            }

            window.update_with_buffer(&buffer, screen_width, screen_height).unwrap();
        }
    });

    loop {
        let mut size_buffer = [0u8; 8];
        stream.read_exact(&mut size_buffer).await?;
        let compressed_size = u64::from_le_bytes(size_buffer) as usize;
        let mut compressed_data = vec![0u8; compressed_size];
        stream.read_exact(&mut compressed_data).await?;
        let decompressed_data = decompress_size_prepended(&compressed_data)?;
        if decompressed_data.len() != max_buffer_size {
            return Err(format!("Invalid image size: {} bytes", decompressed_data.len()).into());
        }
        tx.send(decompressed_data).map_err(|e| format!("Failed to send buffer: {}", e))?;
    }
}
