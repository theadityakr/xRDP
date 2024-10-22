use enigo::{Enigo, MouseControllable};
use serde::{Deserialize};
use tokio::io::{AsyncBufReadExt, BufReader, ReadHalf};
use tokio::net::TcpStream;
use std::io::Result;
use winapi::um::winuser::{VkKeyScanW, INPUT, SendInput, INPUT_KEYBOARD, KEYEVENTF_KEYUP};
use std::mem::size_of;

#[derive(Deserialize, Debug)]
enum InputEvent {
    MouseMove { x: i32, y: i32 },
    MouseClick,
    KeyPress(char),
}

fn send_key_press(ch: char) {
    unsafe {
        let vk_code = VkKeyScanW(ch as u16) as u16;

        // Key down event
        let mut input = INPUT {
            type_: INPUT_KEYBOARD,
            u: std::mem::zeroed(),
        };

        input.u.ki_mut().wVk = vk_code;
        SendInput(1, &mut input, size_of::<INPUT>() as i32);

        // Key up event
        input.u.ki_mut().dwFlags = KEYEVENTF_KEYUP;
        SendInput(1, &mut input, size_of::<INPUT>() as i32);
    }
}

pub async fn read_and_apply_input(mut read_half: ReadHalf<TcpStream>) -> Result<()> {
    let mut enigo = Enigo::new();
    let reader = BufReader::new(read_half);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        println!("Received data from client: {}", line);

        if let Ok(event) = serde_json::from_str::<InputEvent>(&line) {
            println!("Deserialized event: {:?}", event);

            match event {
                InputEvent::MouseMove { x, y } => {
                    println!("Moving mouse to ({}, {})", x, y);
                    enigo.mouse_move_to(x, y);
                }
                InputEvent::MouseClick => {
                    println!("Mouse click event");
                    enigo.mouse_click(enigo::MouseButton::Left);
                }
                InputEvent::KeyPress(ch) => {
                    println!("Key press event: '{}'", ch);
                    send_key_press(ch);  // Use winapi for key presses
                }
            }
        }
    }

    Ok(())
}
