
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream};
use tokio::sync::mpsc::*;
use tokio::sync::oneshot;

use crate::parser::*;
use crate::protocol::*;

pub async fn process(mut socket: TcpStream, tx: Sender<CommandWrapper>) {

    let mut buf = vec![0; 4*1024];

    loop {
        let n = match socket.read(&mut buf).await {
            Ok(0) => {
                // Connection closed by client
                return;
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read data from socket: {}", e);
                return;
            }
        };

        // Parse all commands from the buffer (handles pipelining)
        let commands = parse_multiple_commands(&buf[0..n]);

        // Process each command and collect responses
        for cmd in commands {
            let res = send_command(tx.clone(), cmd).await;
            let value = map_response_to_resp(res);

            if let Err(e) = socket.write_all(value.encode().as_slice()).await {
                eprintln!("Failed to write data to socket: {}", e);
                return;
            }
        }
    }
}

async fn send_command(tx: Sender<CommandWrapper>, cmd: Command) -> Response {
    
    let (resp_tx, resp_rx) = oneshot::channel::<Response>();

    let send = tx.send(CommandWrapper{ cmd : cmd, resp : resp_tx}).await;

    let res = resp_rx.await.unwrap_or(Response::Error{msg : "".into()});
    println!("{}",res);
    res
}