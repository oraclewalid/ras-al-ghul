
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream};
use tokio::sync::mpsc::*;
use tokio::sync::oneshot;

use crate::parser::*;
use crate::protocol::*;

pub async fn process(mut socket: TcpStream, tx: Sender<CommandWrapper>) {

    let mut buf = vec![0; 4*1024];

    let n = socket
        .read(&mut buf)
        .await
        .expect("failed to read data from socket");

    let cmd = parse_and_map_to_command(&buf[0..(n)]);

    let res = send_command(tx, cmd).await;
    let value = map_response_to_resp(res);

    socket
        .write_all(value.encode().as_slice())
        .await
        .expect("failed to write data to socket");
    
    //tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
}

async fn send_command(tx: Sender<CommandWrapper>, cmd: Command) -> Response {
    
    let (resp_tx, resp_rx) = oneshot::channel::<Response>();

    let send = tx.send(CommandWrapper{ cmd : cmd, resp : resp_tx}).await;

    let res = resp_rx.await.unwrap_or(Response::Error{msg : "".into()});
    tracing::info!("{:?}", res);
    res
}