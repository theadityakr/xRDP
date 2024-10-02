// use tokio::net::TcpListener;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use std::error::Error;

// const USERNAME: &str = "Administrator";
// const PASSWORD: &str = "Life@is@2";

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let listener = TcpListener::bind("38.126.136.103:3000").await?;
//     println!("Server listening on 38.126.136.103:3000");

//     loop {
//         let (mut socket, _) = listener.accept().await?;
//         tokio::spawn(async move {
//             let mut buf = [0; 1024];

//             // Read authentication data
//             let n = socket.read(&mut buf).await.unwrap();
//             let auth_data = String::from_utf8_lossy(&buf[0..n]);

//             // Validate credentials (username:password)
//             if auth_data.trim() == format!("{}:{}", USERNAME, PASSWORD) {
//                 println!("Client authenticated successfully");

//                 // Send confirmation
//                 socket.write_all(b"Authenticated").await.unwrap();

//                 // Wait briefly to ensure the client processes the authentication message
//                 tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

//                 // Send mock screen update data (just a text message for now)
//                 let screen_update = b"Screen Buffer: This is a mock screen update";
//                 socket.write_all(screen_update).await.unwrap();
//             } else {
//                 println!("Authentication failed");
//                 socket.write_all(b"Authentication Failed").await.unwrap();
//             }
//         });
//     }
// }

// use scrap::{Capturer, Display};
// use tokio::net::TcpListener;
// use tokio::io::{AsyncWriteExt, AsyncReadExt};
// use std::error::Error;
// use image::{ImageBuffer, Rgba};

// const USERNAME: &str = "Administrator";
// const PASSWORD: &str = "Life@is@2";

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let listener = TcpListener::bind("0.0.0.0:3000").await?;
//     println!("Server listening on 0.0.0.0:3000");

//     loop {
//         let (mut socket, _) = listener.accept().await?;
//         tokio::spawn(async move {
//             let mut buf = [0; 1024];

//             // Read authentication data
//             let n = socket.read(&mut buf).await.unwrap();
//             let auth_data = String::from_utf8_lossy(&buf[0..n]);

//             if auth_data.trim() == format!("{}:{}", USERNAME, PASSWORD) {
//                 println!("Client authenticated successfully");

//                 // Send confirmation
//                 socket.write_all(b"Authenticated").await.unwrap();
//                 tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

//                 // Use `spawn_blocking` to handle the screen capture since `Capturer` is not `Send`
//                 tokio::task::spawn_blocking(move || {
//                     let display = Display::primary().expect("Couldn't find the primary display.");
//                     let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");

//                     // Get width and height before mutable borrow
//                     let width = capturer.width();
//                     let height = capturer.height();

//                     loop {
//                         match capturer.frame() {
//                             Ok(frame) => {
//                                 // Create a new image buffer and copy the captured frame into it
//                                 let mut img_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width as u32, height as u32);
//                                 for (i, pixel) in frame.chunks(4).enumerate() {
//                                     let x = (i % width) as u32;
//                                     let y = (i / width) as u32;
//                                     img_buffer.put_pixel(x, y, Rgba([pixel[2], pixel[1], pixel[0], 255]));
//                                 }

//                                 // Convert image buffer to PNG bytes
//                                 let mut png_bytes = Vec::new();
//                                 image::codecs::png::PngEncoder::new(&mut png_bytes)
//                                     .encode(&img_buffer, width as u32, height as u32, image::ColorType::Rgba8)
//                                     .unwrap();

//                                 // Send the PNG bytes to he client
//                                 tokio::runtime::Handle::current().block_on(async {
//                                     socket.write_all(&(png_bytes.len() as u32).to_be_bytes()).await.unwrap();
//                                     socket.write_all(&png_bytes).await.unwrap();
//                                 });
//                             }
//                             Err(_) => {
//                                 std::thread::sleep(std::time::Duration::from_millis(100));
//                             }
//                         }
//                     }
//                 });
//             } else {
//                 println!("Authentication failed");
//                 socket.write_all(b"Authentication Failed").await.unwrap();
//             }
//         });
//     }
// }

use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::time::Duration;

fn capture_screen() -> Vec<u8> {
    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    loop {
        match capturer.frame() {
            Ok(frame) => {
                // Convert frame to compressed image format (e.g., JPEG)
                let image = image::RgbaImage::from_raw(w as u32, h as u32, frame.to_vec())
                    .expect("Failed to create image from frame");
                let mut buffer = Vec::new();
                image.write_to(&mut buffer, image::ImageOutputFormat::Jpeg(80))
                    .expect("Failed to encode image");
                return buffer;
            }
            Err(error) => {
                if error.kind() == WouldBlock {
                    std::thread::sleep(Duration::from_millis(1));
                    continue;
                }
                panic!("Error: {}", error);
            }
        }
    }
}


use enigo::{Enigo, MouseControllable, KeyboardControllable};

fn handle_input(input: InputEvent) {
    let mut enigo = Enigo::new();
    match input {
        InputEvent::MouseMove { x, y } => enigo.mouse_move_to(x, y),
        InputEvent::MouseClick { button } => enigo.mouse_click(button),
        InputEvent::KeyPress { key } => enigo.key_click(key),
        // Implement other input events
    }
}

use image::jpeg::JpegEncoder;

fn compress_frame(frame: &[u8], width: u32, height: u32) -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut encoder = JpegEncoder::new_with_quality(&mut buffer, 80);
    encoder.encode(frame, width, height, image::ColorType::Rgba8).unwrap();
    buffer
}

use image::jpeg::JpegEncoder;

fn compress_frame(frame: &[u8], width: u32, height: u32) -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut encoder = JpegEncoder::new_with_quality(&mut buffer, 80);
    encoder.encode(frame, width, height, image::ColorType::Rgba8).unwrap();
    buffer
}

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn run_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    while let Ok((socket, _)) = listener.accept().await {
        tokio::spawn(async move {
            handle_client(socket).await;
        });
    }

    Ok(())
}

async fn handle_client(mut socket: TcpStream) {
    // Implement client handling logic here
}