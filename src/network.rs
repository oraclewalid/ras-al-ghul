
use std::{error::Error, fmt::Pointer};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::mpsc::*;
use tokio::sync::oneshot;



use std::env;
mod protocol;
use protocol::*;

mod database;
use database::*;

struct CommandWrapper {
    cmd: Command,
    resp: oneshot::Sender<protocol::Response>,
}

async fn process(mut socket: TcpStream, tx: Sender<CommandWrapper>) {

    let mut buf = vec![0; 4*1024*1024];

    let n = socket
        .read(&mut buf)
        .await
        .expect("failed to read data from socket");

    let cmdRaw = String::from_utf8(buf[0..(n-1)].to_vec()).unwrap();

    let cmd = simple_command_parser(cmdRaw);

    let res = send_cmd_storage_engine(tx, cmd).await;

    socket
        .write_all(res.to_string().as_bytes())
        .await
        .expect("failed to write data to socket");
}

async fn send_cmd_storage_engine(tx: Sender<CommandWrapper>, cmd: Command) -> Response {
    
    let (resp_tx, resp_rx) = oneshot::channel::<protocol::Response>();
    tx.send(CommandWrapper{ cmd : cmd, resp : resp_tx}).await;
    let res = resp_rx.await.unwrap_or(protocol::Response::Error{msg : "".into()});
    println!("{}",res);
    res
}




