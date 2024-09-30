mod network;
mod server;

#[tokio::main]
async fn main()  -> Result<(), Box<dyn std::error::Error>>  {
    network::network_check().await;
    server::server().await?;
    Ok(())
}