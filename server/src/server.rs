use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
// use std::borrow::Borrow;
use std::collections::HashMap;
use scrap::{Capturer, Display};
use std::time::Duration;
use std::thread::sleep;


#[derive(Serialize, Deserialize, Debug)]
struct Credentials {
    username: String,
    password: String,
}

pub async fn server() -> Result<(), Box<dyn std::error::Error>>  {

let listener = TcpListener::bind("38.126.136.103:3000").await?;
    println!("Server running on 38.126.136.103:3000");

    // Storing the valid credentials (for demo purposes)
    let valid_users = HashMap::from([
        ("Administrator".to_string(), "Life@is@2".to_string()),
    ]);

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Connection received from: {}", addr);

        // Read credentials from client
        let mut buffer = [0; 1024];
        let n = socket.read(&mut buffer).await.unwrap();
        let received_data = String::from_utf8_lossy(&buffer[..n]);
        let credentials: Credentials = serde_json::from_str(&received_data).unwrap();
        println!("Received credentials: {:?}", credentials);
        

        let auth_successful = if let Some(valid_password) = valid_users.get(&credentials.username) {
            valid_password == &credentials.password
        } else {
            false
        };
    
        if auth_successful {
            socket.write_all("Authentication successful".as_bytes()).await?;
            socket.flush().await?;
    
                let one_frame_duration = Duration::from_millis(100);
                let display = Display::primary().expect("Couldn't find the primary display.");
                let mut capturer = Capturer::new(display).expect("Couldn't begin capturing.");
    
                loop {
                    match capturer.frame() {
                        Ok(frame) => {
                            // Send the frame to the client
                            if let Err(_) = socket.write_all(&frame).await {
                                println!("Client disconnected.");
                                break;
                            }
                            // Slow down the loop to control frame rate
                            sleep(one_frame_duration);
                        }
                        Err(_) => {
                            // If the screen is being switched or other error, retry
                            // capturer = Capturer::new(display).expect("Couldn't reinitialize capturing.");
                        }
                    }
                }
          
        } else {
            let error_message = if valid_users.contains_key(&credentials.username) {
                "Authentication failed: Incorrect password"
            } else {
                "Authentication failed: Username not found"
            };
            socket.write_all(error_message.as_bytes()).await?;
            socket.flush().await?;
        }

    }
}