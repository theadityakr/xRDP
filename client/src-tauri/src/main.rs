#![allow(unused)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::Arc;
pub mod app {
        pub mod network;
        pub mod client;
        pub mod render_handler;
        // pub mod input;
        pub mod drive;
        pub mod window_handler;
        }
use crate::app::network::network_check;
use crate::app::client::start_client;


#[tauri::command]
async fn connect(connection_settings: String) -> Result<String, String> {

    let connection_settings = Arc::new(connection_settings);
    let result = tokio::spawn(async move {
        start_client(&connection_settings).await
    }).await;

    match result {
        Ok(Ok(auth_success)) => {
            if auth_success {
                Ok("Connection Successful".to_string())
            } else {
                Err("Authentication Failed!".to_string())
            }
        },
        Ok(Err(e)) => Err(format!("Connection Failed: {}", e)),
        Err(e) => Err(format!("Task panicked: {}", e)),
    }

}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![connect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


