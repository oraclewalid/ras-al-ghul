
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

    let memory_manager = tokio::spawn(async move { start_memory_manager(rx).await });

    loop {
        // Asynchronously wait for an inbound socket.
        let (mut socket, _) = listener.accept().await.unwrap();
        let tx2 = tx.clone();

        tokio::spawn(async move { process(socket, tx2).await });
    }
}

async fn process(mut socket: TcpStream, tx: Sender<CommandWrapper>) {

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
}

async fn send_command(tx: Sender<CommandWrapper>, cmd: Command) -> Response {
    
    let (resp_tx, resp_rx) = oneshot::channel::<protocol::Response>();

    let send = tx.send(CommandWrapper{ cmd : cmd, resp : resp_tx}).await;

    let res = resp_rx.await.unwrap_or(protocol::Response::Error{msg : "".into()});
    println!("{}",res);
    res
}

async fn start_memory_manager(mut rx: Receiver<CommandWrapper>) {

    let mut db = database::InMemoryDatabase::new();

    while let Some(cmd_wrapper) = rx.recv().await {

        let cmd = cmd_wrapper.cmd;
        let rx = cmd_wrapper.resp;

        println!("Receive command {}", cmd.clone());

        let response = match cmd {
            Command::Ping => Response::Pong,
            Command::Set{key, value} => {
                db.set(key, value) ;
                Response::OK
            },
            Command::Get{key} => {
                let value = db.get(&key.clone());
                match value {
                    Some(value ) => Response::Get{value: value.clone()},
                    None => Response::Error{msg : format!("The key {} not found", key)}
                }
            }
            _ => Response::Error{msg : "Unknown command".into()}
        };

        let sent_response = rx.send(response);
        match sent_response {
            Result::Ok(_) => println!("Response sent"),
            Result::Err(error) => println!("Error in message sent {}", error)
        }
    }
}

struct CommandWrapper {
    cmd: Command,
    resp: oneshot::Sender<protocol::Response>,
}
