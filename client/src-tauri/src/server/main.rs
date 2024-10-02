// src/main.rs

mod client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <server|client> [args...]", args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "client" => {
            if args.len() < 5 {
                eprintln!("Usage: {} client <address:port> <username> <password>", args[0]);
                std::process::exit(1);
            }
            // Start client and await the response
            let response = client::start_client(&args[2], &args[3], &args[4]).await?;
            println!("Server response: {}", response);
        }
        _ => {
            eprintln!("Invalid command. Use 'server' or 'client'.");
            std::process::exit(1);
        }
    }

    Ok(())
}
