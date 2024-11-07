use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, ReadHalf, AsyncWriteExt, WriteHalf};
use std::error::Error;
use std::sync::mpsc;
use std::thread;
use std::collections::HashSet;
use minifb::{Window, WindowOptions, Key, MouseMode, MouseButton};
use lz4_flex::decompress_size_prepended;

#[derive(Debug)]
enum InputEvent {
    KeyPress(Key),
    KeyRelease(Key),
    MouseMove { x: i32, y: i32 },
    MouseButton { button: MouseButton, pressed: bool },
    MouseScroll(i32),
}

#[cfg(target_os = "windows")]
fn get_screen_resolution() -> (usize, usize) {
    use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
    (unsafe { GetSystemMetrics(SM_CXSCREEN) as usize }, unsafe { GetSystemMetrics(SM_CYSCREEN) as usize })
}

#[cfg(target_os = "macos")]
fn get_screen_resolution() -> (usize, usize) {
    use core_graphics::display::CGDisplay;
    let display = CGDisplay::main();
    (display.pixels_wide() as usize, display.pixels_high() as usize)
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn get_screen_resolution() -> (usize, usize) {
    (1920, 1080)
}

pub async fn render_screen(
    mut read_stream: ReadHalf<TcpStream>,
    write_stream: WriteHalf<TcpStream>
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (tx, rx) = mpsc::channel::<Vec<u8>>();
    let (input_tx, input_rx) = mpsc::channel::<InputEvent>();
    let (screen_width, screen_height) = get_screen_resolution();
    let max_buffer_size: usize = screen_width * screen_height * 4;

    // Spawn a tokio task to handle input events
    let input_handle = tokio::spawn(async move {
        let mut write_stream = write_stream;
        while let Ok(event) = input_rx.recv() {
            let result: Result<(), Box<dyn Error + Send + Sync>> = async {
                match event {
                    InputEvent::KeyPress(key) => {
                        let msg = format!("KEY_PRESS:Key::{:?}\n", key);
                        write_stream.write_all(msg.as_bytes()).await?;
                        write_stream.flush().await?;
                    }
                    InputEvent::KeyRelease(key) => {
                        let msg = format!("KEY_RELEASE:Key::{:?}\n", key);
                        write_stream.write_all(msg.as_bytes()).await?;
                        write_stream.flush().await?;
                    }
                    InputEvent::MouseMove { x, y } => {
                        let msg = format!("MOUSE_MOVE:{},{}\n", x, y);
                        write_stream.write_all(msg.as_bytes()).await?;
                        write_stream.flush().await?;
                    }
                    InputEvent::MouseButton { button, pressed } => {
                        println!("[ InputEvent::MouseButton ] [Debug] [button,pressed]: [{:?},{}]",&button,&pressed);
                        let msg = format!("MOUSE_BUTTON:{:?}:{}\n", button, pressed);
                        write_stream.write_all(msg.as_bytes()).await?;
                        write_stream.flush().await?;
                    }
                    InputEvent::MouseScroll(delta) => {
                        let msg = format!("MOUSE_SCROLL:{}\n", delta);
                        write_stream.write_all(msg.as_bytes()).await?;
                        write_stream.flush().await?;
                    }
                }
                Ok(())
            }.await;

            if let Err(e) = result {
                eprintln!("Error sending input event: {}", e);
                break;
            }
        }
    });

    // Window handling thread with debouncing for key events
    let window_handle = thread::spawn(move || {
        let mut window = Window::new(
            "Remote Desktop Client",
            screen_width,
            screen_height,
            WindowOptions {
                scale: minifb::Scale::X1,
                ..WindowOptions::default()
            },
        ).expect("Failed to create window");

        let mut buffer: Vec<u32> = vec![0; screen_width * screen_height];
        let mut last_mouse_pos = (0, 0);
        let mut pressed_keys = HashSet::new();

        while window.is_open() && !window.is_key_down(Key::Escape) {
            // Update display buffer
            if let Ok(image_buffer) = rx.try_recv() {
                for (i, chunk) in image_buffer.chunks_exact(4).enumerate() {
                    if i < buffer.len() {
                        if let [b, g, r, _] = chunk {
                            buffer[i] = u32::from_le_bytes([*b, *g, *r, 255]);
                        }
                    }
                }
            }

            // Handle keyboard input with debouncing
            for key in window.get_keys() {
                if !pressed_keys.contains(&key) {
                    let _ = input_tx.send(InputEvent::KeyPress(key));
                    pressed_keys.insert(key);
                }
            }
            // Release keys that are no longer pressed
            pressed_keys.retain(|&key| {
                if !window.is_key_down(key) {
                    let _ = input_tx.send(InputEvent::KeyRelease(key));
                    false
                } else {
                    true
                }
            });

            // Handle mouse movement
            if let Some((x, y)) = window.get_mouse_pos(MouseMode::Pass) {
                let x = x as i32;
                let y = y as i32;
                if (x, y) != last_mouse_pos {
                    let _ = input_tx.send(InputEvent::MouseMove { x, y });
                    last_mouse_pos = (x, y);
                }
            }

            // Handle mouse buttons
            for button in [MouseButton::Left, MouseButton::Right, MouseButton::Middle] {
                if window.get_mouse_down(button) {
                    let _ = input_tx.send(InputEvent::MouseButton {
                        button,
                        pressed: true,
                    });
                }
            }

            // Handle mouse scroll
            if let Some(scroll) = window.get_scroll_wheel() {
                let _ = input_tx.send(InputEvent::MouseScroll(scroll.1 as i32));
            }

            window.update_with_buffer(&buffer, screen_width, screen_height).unwrap();
        }
    });

    // Main frame receiving loop
    loop {
        let mut size_buffer = [0u8; 8];
        read_stream.read_exact(&mut size_buffer).await?;
        let compressed_size = u64::from_le_bytes(size_buffer) as usize;
        let mut compressed_data = vec![0u8; compressed_size];
        read_stream.read_exact(&mut compressed_data).await?;
        let decompressed_data = decompress_size_prepended(&compressed_data)?;

        if decompressed_data.len() != max_buffer_size {
            return Err(format!("Invalid image size: {} bytes", decompressed_data.len()).into());
        }

        tx.send(decompressed_data).map_err(|e| format!("Failed to send buffer: {}", e))?;
    }
}
