use tokio::io::{AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use minifb::{Window, Key, MouseButton, MouseMode};
use serde::{Serialize, Deserialize};
use std::collections::HashSet;


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SerializableKey {
    Backspace, Enter, Left, Right, Up, Down, Escape, // Add more keys as needed
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
fn map_key(key: Key) -> Option<SerializableKey> {
    match key {
        Key::Backspace => Some(SerializableKey::Backspace),
        Key::Enter => Some(SerializableKey::Enter),
        Key::Left => Some(SerializableKey::Left),
        Key::Right => Some(SerializableKey::Right),
        Key::Up => Some(SerializableKey::Up),
        Key::Down => Some(SerializableKey::Down),
        Key::Escape => Some(SerializableKey::Escape),
        // Add more mappings as needed
        _ => None,
    }
}

fn map_mouse_button(button: MouseButton) -> SerializableMouseButton {
    match button {
        MouseButton::Left => SerializableMouseButton::Left,
        MouseButton::Right => SerializableMouseButton::Right,
        MouseButton::Middle => SerializableMouseButton::Middle,
    }
}

pub fn capture_input(window: &Window, input_tx: &mpsc::Sender<InputEvent>) -> Result<(), Box<dyn std::error::Error>> {
    static mut LAST_MOUSE_POS: (i32, i32) = (0, 0);
    static mut PRESSED_BUTTONS: Option<HashSet<SerializableMouseButton>> = None;

    unsafe {
        if PRESSED_BUTTONS.is_none() {
            PRESSED_BUTTONS = Some(HashSet::new());
        }
    }

    // Capture keyboard events
    for key in window.get_keys_pressed(minifb::KeyRepeat::No) {
        if let Some(serializable_key) = map_key(key) {
            let event = InputEvent::KeyPress(serializable_key);
            input_tx.try_send(event)?;
        }
    }

    for key in window.get_keys_released() {
        if let Some(serializable_key) = map_key(key) {
            let event = InputEvent::KeyRelease(serializable_key);
            input_tx.try_send(event)?;
        }
    }

    // Capture mouse movement
    if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
        let (last_x, last_y) = unsafe { LAST_MOUSE_POS };
        if x as i32 != last_x || y as i32 != last_y {
            let event = InputEvent::MouseMove { x: x as i32, y: y as i32 };
            input_tx.try_send(event)?;
            unsafe { LAST_MOUSE_POS = (x as i32, y as i32); }
        }
    }

    // Capture mouse button events
    unsafe {
        if let Some(ref mut pressed_buttons) = PRESSED_BUTTONS {
            for button in &[MouseButton::Left, MouseButton::Right, MouseButton::Middle] {
                let serializable_button = map_mouse_button(*button);
                if window.get_mouse_down(*button) {
                    if pressed_buttons.insert(serializable_button) {
                        let event = InputEvent::MouseButtonPress(serializable_button);
                        input_tx.try_send(event)?;
                    }
                } else {
                    if pressed_buttons.remove(&serializable_button) {
                        let event = InputEvent::MouseButtonRelease(serializable_button);
                        input_tx.try_send(event)?;
                    }
                }
            }
        }
    }

    Ok(())
}

pub async fn send_input_events(mut write_stream: WriteHalf<TcpStream>, mut input_rx: mpsc::Receiver<InputEvent>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    while let Some(event) = input_rx.recv().await {
        let serialized = bincode::serialize(&event)?;
        let len = serialized.len() as u32;
        write_stream.write_all(&len.to_le_bytes()).await?;
        write_stream.write_all(&serialized).await?;
        write_stream.flush().await?;
    }
    Ok(())
}