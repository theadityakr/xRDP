use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
use serde_json::Result as JsonResult;
use minifb::{Key, Window, WindowOptions};
// use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
// use winit::event_loop::{ControlFlow, EventLoop};

use crate::app::render;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ConnectionSettings {
    computer: String,
    username: String,
    password: String,
    general_settings: GeneralSettings,
    advanced_settings: AdvancedSettings,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GeneralSettings {
    save_password: bool,
    multiple_display: bool,
    local_drives_redirection: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AdvancedSettings {
    printers: bool,
    clipboard: bool,
}

#[derive(Debug)]
struct Credentials {
    address: String,
    username: String,
    password: String,
}

async fn initial_check(connection_settings: String) -> JsonResult<Credentials> {
    let settings: ConnectionSettings = serde_json::from_str(&connection_settings)?;
    
    Ok(Credentials {
        address: settings.computer,
        username: settings.username,
        password: settings.password
    })
}

pub async fn start_client(connection_settings: String) -> Result<String, Box<dyn std::error::Error>> {
    let credentials = initial_check(connection_settings).await?;

    let mut stream = TcpStream::connect(&credentials.address).await?;
    println!("Connected to the server at {}", &credentials.address);

    render::render_screen(stream).await?;
    // let creds_json = serde_json::to_string(&credentials)?;
    // stream.write_all(creds_json.as_bytes()).await?;
    // stream.flush().await?;

    // // Read server's response
    // let mut buffer = [0; 1024];
    // let n = stream.read(&mut buffer).await?;
    // let response = String::from_utf8_lossy(&buffer[..n]).to_string();

    // use winit::event_loop::EventLoop;
    // let event_loop = EventLoop::new().unwrap();

    // let mut window = Window::new("Remote Desktop Client", 800, 600, WindowOptions::default())
    //     .expect("Unable to open window");
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // let mut buffer: Vec<u32> = vec![0; 800 * 600]; 
    

    // event_loop.run(move |event, _, control_flow| {
    //     *control_flow = ControlFlow::Poll;

    //     match event {
    //         Event::WindowEvent { event, .. } => match event {
    //             WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
    //             WindowEvent::KeyboardInput { input, .. } => {
    //                 if let KeyboardInput {
    //                     state: ElementState::Pressed,
    //                     virtual_keycode: Some(VirtualKeyCode::Escape),
    //                     ..
    //                 } = input
    //                 {
    //                     *control_flow = ControlFlow::Exit;
    //                 }
    //             }
    //             WindowEvent::CursorMoved { position, .. } => {
    //                 println!("Mouse moved to: {:?}", position);
    //                 // Here you'd send the cursor movement to the server
    //             }
    //             _ => (),
    //         },
    //         Event::MainEventsCleared => {
    //             // Check for new frame from server
    //             let mut frame_buffer = vec![0u8; 800 * 600 * 4];
    //             let n = stream.read(&mut frame_buffer).unwrap_or(0);

    //             if n > 0 {
    //                 // Copy data from the received buffer to the window buffer
    //                 for (i, chunk) in frame_buffer.chunks_exact(4).enumerate() {
    //                     let pixel = u32::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
    //                     buffer[i] = pixel;
    //                 }

    //                 // Update the window with the new frame
    //                 window.update_with_buffer(&buffer, 800, 600).unwrap();
    //             }
    //         }
    //         _ => (),
    //     }
    // });

    
    Ok(credentials.address)
}



