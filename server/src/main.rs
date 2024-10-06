#![allow(unused)]

mod app{
    pub mod server;
    pub mod helper;
    pub mod auth;
    pub mod read_inputs;
}

use crate::app::server::server;

#[tokio::main]
async fn main()  -> Result<(), Box<dyn std::error::Error>>  {
    server().await?;
    Ok(())
}