#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod app {
        pub mod network;
        pub mod client;
        pub mod render;
    }
use crate::app::network::network_check;
use crate::app::client::start_client;


#[tauri::command]
async fn connect(connection_settings: String)  {

    // get data store in the VM using client hostname folder XXXX/netw...2024..txt
    // match network_check().await {
    //     Ok(_) => println!("Network check completed successfully"),
    //     Err(e) => eprintln!("Network check failed: {}", e),
    // }

    match start_client(connection_settings).await {
        Ok(_) => println!("Connection Started successfully"),
        Err(e) => eprintln!("Connection Failed: {}", e),
    }

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![connect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


