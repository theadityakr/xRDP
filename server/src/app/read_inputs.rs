use tokio::io::{AsyncReadExt, ReadHalf};
use tokio::net::TcpStream;
use serde::{Serialize, Deserialize};
use std::error::Error;
use minifb::{Window, WindowOptions, Key, MouseButton, MouseMode};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SerializableKey {
    Backspace, Enter, Left, Right, Up, Down, Escape, // Add more keys as needed
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SerializableMouseButton {
    Left, Right, Middle,
}

#[derive(Serialize, Deserialize)]
pub enum InputEvent {
    KeyPress(SerializableKey),
    KeyRelease(SerializableKey),
    MouseMove { x: i32, y: i32 },
    MouseButtonPress(SerializableMouseButton),
    MouseButtonRelease(SerializableMouseButton),
}

pub async fn read_user_input_make_changes(mut read_half: ReadHalf<TcpStream>) -> Result<(), Box<dyn Error + Send + Sync>> {
    loop {
        // Read the length of the serialized data
        let mut len_bytes = [0u8; 4];
        read_half.read_exact(&mut len_bytes).await?;
        let len = u32::from_le_bytes(len_bytes) as usize;

        // Read the serialized data
        let mut buffer = vec![0u8; len];
        read_half.read_exact(&mut buffer).await?;

        // Deserialize the input event
        let event: InputEvent = bincode::deserialize(&buffer)?;

        // Process the input event
        match event {
            InputEvent::KeyPress(key) => {
                println!("Key pressed: {:?}", key);
                // Implement key press logic here
            }
            InputEvent::KeyRelease(key) => {
                println!("Key released: {:?}", key);
                // Implement key release logic here
            }
            InputEvent::MouseMove { x, y } => {
                println!("Mouse moved to: ({}, {})", x, y);
                // Implement mouse move logic here
            }
            InputEvent::MouseButtonPress(button) => {
                println!("Mouse button pressed: {:?}", button);
                // Implement mouse button press logic here
            }
            InputEvent::MouseButtonRelease(button) => {
                println!("Mouse button released: {:?}", button);
                // Implement mouse button release logic here
            }
        }

        // Here you would typically update the server's state based on the input
        // For example, moving the cursor, clicking, or typing in applications
    }
}