use tokio::io::{AsyncReadExt, BufReader, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use image::{ImageBuffer, Rgb};
use minifb::{Window, WindowOptions, Key};
use std::error::Error;
use std::sync::Arc;

pub async fn render_screen(stream: ReadHalf<TcpStream>) -> Result<(), Box<dyn Error + Send + Sync>> {
    Ok(())  
}