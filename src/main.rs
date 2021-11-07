
use std::{error::Error};
use tokio::net::{TcpListener};
use tokio::sync::mpsc;


use std::env;
mod protocol;
use protocol::*;

mod database;

mod parser;

mod database_manager;
mod config;
mod network;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let addr = env::args()
        .nth(1);

    let conf = config::get_config(addr);

    let listener = TcpListener::bind(conf.server.to_server_with_port()).await?;

    println!("Listening on: {}", conf.server.to_server_with_port());

    let (tx, rx) = mpsc::channel::<CommandWrapper>(32);

    tokio::spawn(async move { database_manager::start_memory_manager(rx, conf.clone()).await });

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let tx2 = tx.clone();

        tokio::spawn(async move { network::process(socket, tx2).await });
    }
}