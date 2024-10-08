use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
use serde_json::Result as JsonResult;
use tokio::task;
use tokio::io::{ReadHalf, WriteHalf};
use std::error::Error;

use crate::app::render;
use crate::app::input;


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

    let credentials = initial_check(connection_settings.clone()).await?;
    let mut stream = TcpStream::connect(&credentials.address).await?;
    println!("Connected to the server at {}", &credentials.address);
    stream.write_all(connection_settings.as_bytes()).await?;

    let (read_half, write_half) = stream.into_split();

    let render_task = task::spawn(async move {
        if let Err(e) = render::render_screen(read_half).await {
            eprintln!("Failed to render screen: {}", e);
        }
    });

    // let input_task = task::spawn(async move {
    //     if let Err(e) = input::capture_and_send_input(write_half).await {
    //         eprintln!("Failed to capture input: {}", e);
    //     }
    // });

    // let _ = tokio::join!(render_task, input_task);

    Ok(credentials.address)
}


