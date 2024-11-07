use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, ReadHalf};
use std::error::Error;
use minifb::{Key, MouseButton};

#[cfg(target_os = "windows")]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_0, INPUT_TYPE, MOUSEINPUT, KEYBDINPUT,
    SendInput, VIRTUAL_KEY, VK_CONTROL, VK_MENU, VK_SHIFT,
    KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE,
    MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_MOVE,
    MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
    MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,
    MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP,
    MOUSEEVENTF_WHEEL,
};

fn parse_key(key_str: &str) -> Option<Key> {
    match key_str {
        // Alphanumeric keys
        "Key::A" => Some(Key::A), "Key::B" => Some(Key::B), "Key::C" => Some(Key::C),
        "Key::D" => Some(Key::D), "Key::E" => Some(Key::E), "Key::F" => Some(Key::F),
        "Key::G" => Some(Key::G), "Key::H" => Some(Key::H), "Key::I" => Some(Key::I),
        "Key::J" => Some(Key::J), "Key::K" => Some(Key::K), "Key::L" => Some(Key::L),
        "Key::M" => Some(Key::M), "Key::N" => Some(Key::N), "Key::O" => Some(Key::O),
        "Key::P" => Some(Key::P), "Key::Q" => Some(Key::Q), "Key::R" => Some(Key::R),
        "Key::S" => Some(Key::S), "Key::T" => Some(Key::T), "Key::U" => Some(Key::U),
        "Key::V" => Some(Key::V), "Key::W" => Some(Key::W), "Key::X" => Some(Key::X),
        "Key::Y" => Some(Key::Y), "Key::Z" => Some(Key::Z), "Key::0" => Some(Key::Key0),
        "Key::1" => Some(Key::Key1), "Key::2" => Some(Key::Key2), "Key::3" => Some(Key::Key3),
        "Key::4" => Some(Key::Key4), "Key::5" => Some(Key::Key5), "Key::6" => Some(Key::Key6),
        "Key::7" => Some(Key::Key7), "Key::8" => Some(Key::Key8), "Key::9" => Some(Key::Key9),

        // Function keys
        "Key::F1" => Some(Key::F1), "Key::F2" => Some(Key::F2), "Key::F3" => Some(Key::F3),
        "Key::F4" => Some(Key::F4), "Key::F5" => Some(Key::F5), "Key::F6" => Some(Key::F6),
        "Key::F7" => Some(Key::F7), "Key::F8" => Some(Key::F8), "Key::F9" => Some(Key::F9),
        "Key::F10" => Some(Key::F10), "Key::F11" => Some(Key::F11), "Key::F12" => Some(Key::F12),

        // Navigation and control keys
        "Key::Space" => Some(Key::Space), "Key::Enter" => Some(Key::Enter),
        "Key::Backspace" => Some(Key::Backspace), "Key::Tab" => Some(Key::Tab),
        "Key::Escape" => Some(Key::Escape), "Key::Delete" => Some(Key::Delete),
        "Key::Insert" => Some(Key::Insert), "Key::Home" => Some(Key::Home),
        "Key::End" => Some(Key::End), "Key::PageUp" => Some(Key::PageUp),
        "Key::PageDown" => Some(Key::PageDown), "Key::Left" => Some(Key::Left),
        "Key::Right" => Some(Key::Right), "Key::Up" => Some(Key::Up),
        "Key::Down" => Some(Key::Down),

        // Numeric keypad
        "Key::NumPad0" => Some(Key::NumPad0), "Key::NumPad1" => Some(Key::NumPad1),
        "Key::NumPad2" => Some(Key::NumPad2), "Key::NumPad3" => Some(Key::NumPad3),
        "Key::NumPad4" => Some(Key::NumPad4), "Key::NumPad5" => Some(Key::NumPad5),
        "Key::NumPad6" => Some(Key::NumPad6), "Key::NumPad7" => Some(Key::NumPad7),
        "Key::NumPad8" => Some(Key::NumPad8), "Key::NumPad9" => Some(Key::NumPad9),
        "Key::NumPadDot" => Some(Key::NumPadDot), "Key::NumPadEnter" => Some(Key::NumPadEnter),
        "Key::NumPadPlus" => Some(Key::NumPadPlus), "Key::NumPadMinus" => Some(Key::NumPadMinus),
        "Key::NumPadMultiply" => Some(Key::NumPadAsterisk), "Key::NumPadDivide" => Some(Key::NumPadSlash),

        // Modifier keys
        "Key::LeftShift" => Some(Key::LeftShift), "Key::RightShift" => Some(Key::RightShift),
        "Key::LeftCtrl" => Some(Key::LeftCtrl), "Key::RightCtrl" => Some(Key::RightCtrl),
        "Key::LeftAlt" => Some(Key::LeftAlt), "Key::RightAlt" => Some(Key::RightAlt),
        // "Key::LeftMeta" => Some(Key::LeftMeta), "Key::RightMeta" => Some(Key::RightMeta),

        // Special characters and punctuation
        "Key::Apostrophe" => Some(Key::Apostrophe), "Key::Comma" => Some(Key::Comma),
        "Key::Period" => Some(Key::Period), "Key::Slash" => Some(Key::Slash),
        "Key::Semicolon" => Some(Key::Semicolon), "Key::Equal" => Some(Key::Equal),
        "Key::Minus" => Some(Key::Minus), "Key::LeftBracket" => Some(Key::LeftBracket),
        "Key::RightBracket" => Some(Key::RightBracket), "Key::Backslash" => Some(Key::Backslash),
        // "Key::Grave" => Some(Key::Grave),

        // Additional punctuation and symbols
        // "Key::Exclamation" => Some(Key::Exclamation), "Key::At" => Some(Key::At),
        // "Key::Hash" => Some(Key::Hash), "Key::Dollar" => Some(Key::Dollar),
        // "Key::Percent" => Some(Key::Percent), "Key::Caret" => Some(Key::Caret),
        // "Key::Ampersand" => Some(Key::Ampersand), "Key::Asterisk" => Some(Key::Asterisk),
        // "Key::LeftParen" => Some(Key::LeftParen), "Key::RightParen" => Some(Key::RightParen),
        // "Key::Underscore" => Some(Key::Underscore), "Key::Plus" => Some(Key::Plus),
        // "Key::Colon" => Some(Key::Colon), "Key::Quote" => Some(Key::Quote),

        _ => None,
    }
}

fn parse_mouse_button(button_str: &str) -> Option<MouseButton> {
    match button_str {
        "Left" => Some(MouseButton::Left),
        "Right" => Some(MouseButton::Right),
        "Middle" => Some(MouseButton::Middle),
        _ => None,
    }
}

#[cfg(target_os = "windows")]
fn key_to_virtual_key(key: Key) -> VIRTUAL_KEY {
    use windows::Win32::UI::Input::KeyboardAndMouse::*;
    match key {
        Key::A => VK_A,
        Key::B => VK_B,
        Key::C => VK_C,
        Key::D => VK_D,
        Key::E => VK_E,
        Key::F => VK_F,
        Key::G => VK_G,
        Key::H => VK_H,
        Key::I => VK_I,
        Key::J => VK_J,
        Key::K => VK_K,
        Key::L => VK_L,
        Key::M => VK_M,
        Key::N => VK_N,
        Key::O => VK_O,
        Key::P => VK_P,
        Key::Q => VK_Q,
        Key::R => VK_R,
        Key::S => VK_S,
        Key::T => VK_T,
        Key::U => VK_U,
        Key::V => VK_V,
        Key::W => VK_W,
        Key::X => VK_X,
        Key::Y => VK_Y,
        Key::Z => VK_Z,
        Key::F1 => VK_F1,
        Key::F2 => VK_F2,
        Key::F3 => VK_F3,
        Key::F4 => VK_F4,
        Key::F5 => VK_F5,
        Key::F6 => VK_F6,
        Key::F7 => VK_F7,
        Key::F8 => VK_F8,
        Key::F9 => VK_F9,
        Key::F10 => VK_F10,
        Key::F11 => VK_F11,
        Key::F12 => VK_F12,
        Key::Space => VK_SPACE,
        Key::Enter => VK_RETURN,
        Key::Backspace => VK_BACK,
        Key::Tab => VK_TAB,
        Key::Escape => VK_ESCAPE,
        Key::Delete => VK_DELETE,
        Key::Left => VK_LEFT,
        Key::Right => VK_RIGHT,
        Key::Up => VK_UP,
        Key::Down => VK_DOWN,
        Key::NumPad0 => VK_NUMPAD0,
        Key::NumPad1 => VK_NUMPAD1,
        Key::NumPad2 => VK_NUMPAD2,
        Key::NumPad3 => VK_NUMPAD3,
        Key::NumPad4 => VK_NUMPAD4,
        Key::NumPad5 => VK_NUMPAD5,
        Key::NumPad6 => VK_NUMPAD6,
        Key::NumPad7 => VK_NUMPAD7,
        Key::NumPad8 => VK_NUMPAD8,
        Key::NumPad9 => VK_NUMPAD9,
        Key::LeftShift | Key::RightShift => VK_SHIFT,
        Key::LeftCtrl | Key::RightCtrl => VK_CONTROL,
        Key::LeftAlt | Key::RightAlt => VK_MENU,
        _ => VIRTUAL_KEY(0),
    }
}

#[cfg(target_os = "windows")]
async fn simulate_key_event(key: Key, press: bool) {
    let vk = key_to_virtual_key(key);
    if vk.0 == 0 {
        return;
    }

    let mut input = INPUT {
        r#type: INPUT_TYPE(1), // INPUT_KEYBOARD
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: 0,
                dwFlags: if press { Default::default() } else { KEYEVENTF_KEYUP },
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    unsafe {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

#[cfg(target_os = "windows")]
pub fn get_screen_resolution() -> (i32, i32) {
    use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
    (unsafe { GetSystemMetrics(SM_CXSCREEN) as i32 }, unsafe { GetSystemMetrics(SM_CYSCREEN) as i32 })
}

#[cfg(target_os = "windows")]
async fn simulate_mouse_move(screen_width: i32, screen_height: i32, x: i32, y: i32) {
    println!("[simulate_mouse_move] [Debug] [x,y]: [{},{}]", x, y);
    
    let input = INPUT {
        r#type: INPUT_TYPE(0), // INPUT_MOUSE
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: (x * 65535) / screen_width,
                dy: (y * 65535) / screen_height,
                mouseData: 0,
                dwFlags: MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    unsafe {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

#[cfg(target_os = "windows")]
async fn simulate_mouse_button(button: MouseButton, pressed: bool) {
    println!("[simulate_mouse_button] [Debug] [button,pressed]: [{:?},{}]",&button,&pressed);
    let flags = match (button, pressed) {
        (MouseButton::Left, true) => MOUSEEVENTF_LEFTDOWN,
        (MouseButton::Left, false) => MOUSEEVENTF_LEFTUP,
        (MouseButton::Right, true) => MOUSEEVENTF_RIGHTDOWN,
        (MouseButton::Right, false) => MOUSEEVENTF_RIGHTUP,
        (MouseButton::Middle, true) => MOUSEEVENTF_MIDDLEDOWN,
        (MouseButton::Middle, false) => MOUSEEVENTF_MIDDLEUP,
    };

    let input = INPUT {
        r#type: INPUT_TYPE(0), // INPUT_MOUSE
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: 0,
                dy: 0,
                mouseData: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    unsafe {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

#[cfg(target_os = "windows")]
async fn simulate_mouse_scroll(delta: i32) {
    let input = INPUT {
        r#type: INPUT_TYPE(0), // INPUT_MOUSE
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: 0,
                dy: 0,
                mouseData: (delta * 120) as i32, // Windows expects multiples of 120
                dwFlags: MOUSEEVENTF_WHEEL,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    unsafe {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

pub async fn read_and_apply_input(mut read_half: ReadHalf<TcpStream>) -> Result<(), Box<dyn Error>> {
    println!("[read_and_apply_input] [Debug] : Reading Input");
    let mut buffer = String::new();
    let mut tmp = [0u8; 1024];
    let (screen_width, screen_height) = get_screen_resolution();

    loop {
        let n = read_half.read(&mut tmp).await?;
        if n == 0 {
            return Ok(());
        }
        buffer.push_str(&String::from_utf8_lossy(&tmp[..n]));

        while let Some(pos) = buffer.find('\n') {
            let command = buffer[..pos].to_string();
            buffer = buffer[pos + 1..].to_string();

            // Parse and handle the command
            if let Some((cmd_type, args)) = command.split_once(':') {
                match cmd_type {
                    "KEY_PRESS" => {
                        if let Some(key) = parse_key(args) {
                            simulate_key_event(key, true).await;
                        }
                    }
                    "KEY_RELEASE" => {
                        if let Some(key) = parse_key(args) {
                            simulate_key_event(key, false).await;
                        }
                    }
                    "MOUSE_MOVE" => {
                        if let Some((x, y)) = args.split_once(',') {
                            if let (Ok(x), Ok(y)) = (x.parse::<i32>(), y.parse::<i32>()) {
                                simulate_mouse_move(screen_width, screen_height, x, y).await;
                            }
                        }
                    }
                    "MOUSE_BUTTON" => {
                        if let Some((button, pressed)) = args.split_once(':') {
                            println!("[read_and_apply_input] [Debug] [MOUSE_BUTTON] [buttton,pressed]: {:?},{}",button,pressed);
                            if let (Some(button), Ok(pressed)) = (
                                parse_mouse_button(button),
                                pressed.parse::<bool>()
                            ) {
                                simulate_mouse_button(button, pressed).await;
                            }
                        }
                    }
                    "MOUSE_SCROLL" => {
                        if let Ok(delta) = args.parse::<i32>() {
                            simulate_mouse_scroll(delta).await;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
