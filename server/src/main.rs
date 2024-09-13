use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use image::{ImageBuffer, Rgb};
use screenshots::Screen;
use inputbot::{MouseCursor, KeybdKey};
use std::error::Error;
// use std::async::Arc;

struct RDPServer {
    host: String,
    port: u16,
}

impl RDPServer {
    fn new(host: String, port: u16) -> Self {
        RDPServer { host, port }
    }

    async fn start(&self) -> Result<(), Box<dyn Error>> {
        let addr = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(&addr).await?;
        println!("Server listening on {}", addr);

        loop {
            let (socket, addr) = listener.accept().await?;
            println!("Connection from {}", addr);
            tokio::spawn(async move {
                if let Err(e) = handle_client(socket).await {
                    eprintln!("Error handling client: {}", e);
                }
            });
        }
    }
}

async fn handle_client(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let screen = Screen::from_point(0, 0).unwrap();
    
    loop {
        // Capture screen
        let image = screen.capture().unwrap();
        let rgb_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(
            image.width() as u32,
            image.height() as u32,
            image.to_rgb8().into_raw(),
        ).unwrap();

        // Encode image to JPEG
        let mut jpeg_data = Vec::new();
        let mut encoder = jpeg_encoder::Encoder::new(&mut jpeg_data, 50);
        encoder.encode(
            &rgb_image.into_raw(),
            rgb_image.width() as u16,
            rgb_image.height() as u16,
            jpeg_encoder::ColorType::Rgb,
        )?;

        // Send image size and data
        socket.write_all(&(jpeg_data.len() as u32).to_be_bytes()).await?;
        socket.write_all(&jpeg_data).await?;

        // Receive input events
        let mut event_type = [0u8; 1];
        if socket.read_exact(&mut event_type).await.is_err() {
            break;
        }

        match event_type[0] as char {
            'm' => {
                let mut coords = [0u8; 8];
                socket.read_exact(&mut coords).await?;
                let coords_str = String::from_utf8_lossy(&coords);
                let (x, y) = coords_str.split_once(',').unwrap();
                let (x, y) = (x.parse::<i32>()?, y.parse::<i32>()?);
                MouseCursor::move_abs(x, y);
            }
            'c' => {
                MouseCursor::click();
            }
            'k' => {
                let mut key = [0u8; 1];
                socket.read_exact(&mut key).await?;
                let key_char = key[0] as char;
                if let Some(key) = char_to_keybd_key(key_char) {
                    key.press();
                    key.release();
                }
            }
            _ => break,
        }
    }

    Ok(())
}

fn char_to_keybd_key(c: char) -> Option<KeybdKey> {
    match c {
        'a'..='z' => Some(KeybdKey::from_char(c.to_ascii_uppercase())),
        '0'..='9' => Some(KeybdKey::from_char(c)),
        // Add more mappings as needed
        _ => None,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting.......");
    let server = RDPServer::new("0.0.0.0".to_string(), 5000);
    server.start().await
}