
use std::{error::Error};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::mpsc::*;
use tokio::sync::oneshot;


use std::env;
mod protocol;
use protocol::*;

mod database;

mod parser;
use parser::*;

mod database_manager;
mod network;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6543".to_string());

    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.
    let listener = TcpListener::bind(&addr).await?;

    println!("Listening on: {}", addr);

    let (tx, rx) = mpsc::channel::<CommandWrapper>(32);

    tokio::spawn(async move { database_manager::start_memory_manager(rx).await });

    loop {
        // Asynchronously wait for an inbound socket.
        let (mut socket, _) = listener.accept().await.unwrap();
        let tx2 = tx.clone();

        tokio::spawn(async move { network::process(socket, tx2).await });
    }
}