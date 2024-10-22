use enigo::{Enigo, KeyboardControllable, MouseControllable};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, ReadHalf, AsyncWriteExt, WriteHalf};
use tokio::task;
use winit::{
    event::{Event, WindowEvent, ElementState, MouseButton, KeyboardInput},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};
use winit::{
    event_loop::{EventLoopBuilder},
    platform::windows::EventLoopBuilderExtWindows, 
};
use std::error::Error;

pub async fn capture_and_send_input(mut write_half: WriteHalf<TcpStream>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut enigo = Enigo::new();
    let mut is_focused = false;

    let mut event_loop = EventLoopBuilder::new().with_any_thread(true).build();
    let window = WindowBuilder::new()
        .with_title("Input Capture Test")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)
        .expect("Failed to create window");

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent { event: WindowEvent::Focused(focused), .. } => {
                is_focused = focused;
            }
            Event::WindowEvent { event: WindowEvent::CursorMoved { position, .. }, .. } => {
                if is_focused {
                    let x = position.x as i32;
                    let y = position.y as i32;
                    let msg = format!("MouseMoved: {},{}\n", x, y); // Newline added for clarity
                    // Await the write operation
                    // let _ = write_half.write_all(msg.as_bytes()).await;
                }
            }
            Event::WindowEvent { event: WindowEvent::MouseInput { state, button, .. }, .. } => {
                if is_focused {
                    let msg = format!("MouseButton: {:?}, State: {:?}\n", button, state);
                    // Await the write operation
                    // let _ = tokio::spawn(async move {
                    //     let _ = write_half.write_all(msg.as_bytes()).await;
                    // });
                }
            }
            Event::WindowEvent { event: WindowEvent::KeyboardInput { input: KeyboardInput { state, virtual_keycode, .. }, .. }, .. } => {
                if is_focused {
                    let key = virtual_keycode.map(|k| format!("{:?}", k)).unwrap_or_else(|| "Unknown".to_string());
                    let msg = format!("Key: {:?}, State: {:?}\n", key, state);
                    // Await the write operation
                    // let _ = write_half.write_all(msg.as_bytes()).await;
                }
            }
            _ => (),
        }
    });
    Ok(())
}