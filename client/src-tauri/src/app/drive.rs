use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ClientDriveInfo {
    pub drive_letter: String,       // Local drive on the client, e.g., "C"
    pub mapped_drive_letter: String, // Drive it should map to on the server, e.g., "Z"
}


