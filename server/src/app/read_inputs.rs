use winapi::um::winuser::{INPUT_u, INPUT, INPUT_MOUSE, INPUT_KEYBOARD, MOUSEINPUT, KEYBDINPUT, SendInput};
use winapi::shared::windef::POINT;
use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, ReadHalf, WriteHalf};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn read_user_input_make_changes(mut stream: ReadHalf<TcpStream>) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0u8; 1024];

    loop {
        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            // Connection closed
            break;
        }

        // Parse the input from the buffer
        let input = parse_input(&buffer[..n])?;

        // Process the input
        match input {
            InputEvent::MouseMove { x, y } => simulate_mouse_move(x, y)?,
            InputEvent::MouseClick { button, down } => simulate_mouse_click(button, down)?,
            InputEvent::KeyPress { key_code, down } => simulate_key_press(key_code, down)?,
        }
    }

    Ok(())
}

enum InputEvent {
    MouseMove { x: i32, y: i32 },
    MouseClick { button: MouseButton, down: bool },
    KeyPress { key_code: u16, down: bool },
}

enum MouseButton {
    Left,
    Right,
    Middle,
}

fn parse_input(buffer: &[u8]) -> Result<InputEvent, Box<dyn std::error::Error>> {
    // Implement parsing logic here
    // This is a placeholder implementation
    Ok(InputEvent::MouseMove { x: 0, y: 0 })
}

fn simulate_mouse_move(x: i32, y: i32) -> Result<(), Box<dyn std::error::Error>> {
    let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    let screen_height = unsafe { GetSystemMetrics(SM_CYSCREEN) };

    let normalized_x = (x as f32 / 65535.0 * screen_width as f32) as i32;
    let normalized_y = (y as f32 / 65535.0 * screen_height as f32) as i32;

    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: unsafe { std::mem::zeroed() },
    };
    unsafe {
        *input.u.mi_mut() = MOUSEINPUT {
            dx: normalized_x,
            dy: normalized_y,
            mouseData: 0,
            dwFlags: winapi::um::winuser::MOUSEEVENTF_MOVE | winapi::um::winuser::MOUSEEVENTF_ABSOLUTE,
            time: 0,
            dwExtraInfo: 0,
        };
    }

    unsafe {
        SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
    }

    Ok(())
}

fn simulate_mouse_click(button: MouseButton, down: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: unsafe { std::mem::zeroed() },
    };

    let (down_flag, up_flag) = match button {
        MouseButton::Left => (winapi::um::winuser::MOUSEEVENTF_LEFTDOWN, winapi::um::winuser::MOUSEEVENTF_LEFTUP),
        MouseButton::Right => (winapi::um::winuser::MOUSEEVENTF_RIGHTDOWN, winapi::um::winuser::MOUSEEVENTF_RIGHTUP),
        MouseButton::Middle => (winapi::um::winuser::MOUSEEVENTF_MIDDLEDOWN, winapi::um::winuser::MOUSEEVENTF_MIDDLEUP),
    };

    unsafe {
        *input.u.mi_mut() = MOUSEINPUT {
            dx: 0,
            dy: 0,
            mouseData: 0,
            dwFlags: if down { down_flag } else { up_flag },
            time: 0,
            dwExtraInfo: 0,
        };
    }

    unsafe {
        SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
    }

    Ok(())
}

fn simulate_key_press(key_code: u16, down: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: unsafe { std::mem::zeroed() },
    };

    unsafe {
        *input.u.ki_mut() = KEYBDINPUT {
            wVk: key_code,
            wScan: 0,
            dwFlags: if down { 0 } else { winapi::um::winuser::KEYEVENTF_KEYUP },
            time: 0,
            dwExtraInfo: 0,
        };
    }

    unsafe {
        SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
    }

    Ok(())
}