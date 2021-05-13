
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
mod config;
mod network;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6543".to_string());

    let conf = config::get_config(addr.clone()).expect("Impossible to parse config file");

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