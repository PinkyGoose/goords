use std::env;

use goords::{Mon, game_server::GameServer};

use log::{info, error, warn, debug};
use tonic::transport::Server;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    
    let server_addr = env::var("SERVER_ADDR").expect("Can't get DB URL");
    // let server_url = env::var("SERVER_PORT").expect("Can't get DB URL");
    let addr=server_addr.parse().expect("Invalid addres");
    info!("Starting Server on {server_addr}");
    
    let haha = Mon::new();
    Server::builder()
        .add_service(GameServer::new(haha))
        .serve(addr)
        .await?;

    Ok(())
}
