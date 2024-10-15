use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, ReadHalf, AsyncWriteExt, WriteHalf};
use std::error::Error;
use std::sync::mpsc;
use std::thread;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    platform::run_return::EventLoopExtRunReturn,
    platform::windows::EventLoopBuilderExtWindows, // Import for `with_any_thread` method
    window::WindowBuilder,
};
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

pub async fn render_screen(
    mut stream: ReadHalf<TcpStream>,
    mut _write_half: WriteHalf<TcpStream>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (tx, rx) = mpsc::channel::<Vec<u8>>();
    let (screen_width, screen_height) = get_screen_resolution();
    let max_buffer_size: usize = screen_width * screen_height * 4;

    // Start the rendering thread using winit.
    thread::spawn(move || {
        // Use `with_any_thread` to allow EventLoop creation on a non-main thread for Windows
        let mut event_loop = EventLoopBuilder::new().with_any_thread(true).build();
        let window = WindowBuilder::new()
            .with_title("Remote Desktop Client")
            .with_inner_size(winit::dpi::LogicalSize::new(screen_width as f64, screen_height as f64))
            .build(&event_loop)
            .unwrap();

        let mut buffer: Vec<u32> = vec![0; screen_width * screen_height];

        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        println!("The close button was pressed; stopping");
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                },
                Event::RedrawRequested(_) => {
                    // Handle window redraw.
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
                        window.request_redraw();
                    }
                }
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                _ => {}
            }
        });
    });

    // Continuously read and decompress data from the stream.
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
