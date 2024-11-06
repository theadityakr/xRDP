use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
use serde_json::Result as JsonResult;
use tokio::task;
use tokio::io::{ReadHalf, WriteHalf};
use std::error::Error;

use crate::app::render_handler;
use crate::app::drive;
use crate::app::window_handler;
// use crate::app::input;


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

pub async fn start_client(connection_settings: &str) -> Result<bool,Box<dyn Error + Send + Sync>> {

    let credentials = initial_check(connection_settings.to_string()).await?;
    let mut stream = TcpStream::connect(&credentials.address).await?;
    println!("[start_client] [Debug] : Connected to the server at {}", &credentials.address);
    stream.write_all(connection_settings.as_bytes()).await?;
    
    let mut buffer = [0u8; 1];
    stream.read_exact(&mut buffer).await?;
    let auth_success: bool = buffer[0] != 0;
    if !auth_success {
        return Ok(false);
    }

    let mut session_id_bytes = [0u8; 4]; 
    stream.read_exact(&mut session_id_bytes).await?;
    let session_id = u32::from_le_bytes(session_id_bytes);
    println!("[start_client] [Debug] [session_id] : {}", session_id);


    // let client_drive_info = drive::ClientDriveInfo {
    //     drive_letter: "C".to_string(),
    //     mapped_drive_letter: "Z".to_string(),
    // };
    // let serialized_drive_info = serde_json::to_vec(&client_drive_info)?;
    // stream.write_all(&serialized_drive_info).await?;
    // println!("Drive information sent to the server.");


    let (read_half, write_half) =  tokio::io::split(stream);

    task::spawn(async move {
        if let Err(e) = window_handler::render_screen(read_half, write_half).await {
            eprintln!("Failed to render screen: {}", e);
        }
    });

    // let input_task = task::spawn(async move {
    //     if let Err(e) = input::capture_and_send_input(write_half).await {
    //         eprintln!("Failed to capture input: {}", e);
    //     }
    // });

    Ok(true)
}