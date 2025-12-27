
use std::{error::Error};
use tokio::net::{TcpListener};
use tokio::sync::mpsc;


use std::env;
mod protocol;
use protocol::*;

mod telemetry;

mod database;

mod parser;

mod database_manager;
mod config;
mod network;
mod scheduler;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let addr = env::args()
        .nth(1);

    let conf = config::get_config(addr);

    telemetry::init();

    let listener = TcpListener::bind(conf.server.to_server_with_port()).await?;

    tracing::info!("Listening on: {}", conf.server.to_server_with_port());

    let (tx, rx) = mpsc::channel::<CommandWrapper>(32);
    let tx2 = tx.clone();
    let conf2 = conf.clone();
    tokio::spawn(async move { database_manager::start_memory_manager(rx, conf.clone()).await });
    tokio::spawn(async move { scheduler::start_persistance_cron(tx2, conf2).await });

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let tx2 = tx.clone();

        tokio::spawn(async move { network::process(socket, tx2).await });
    }
}