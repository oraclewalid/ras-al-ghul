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

async fn start_db_engine(mut rx: Receiver<CommandWrapper>) {


    while let Some(cmd_wrapper) = rx.recv().await {

        let cmd = cmd_wrapper.cmd;
        let rx = cmd_wrapper.resp;

        println!("Receive command {}", cmd.clone());

        
        let sentRs = rx.send(response);
        match sentRs {
            Result::Ok(v) => println!("Response sent"),
            Result::Err(error) => println!("Error in message sent {}", error)
        }
    }
}

fn execute_command(cmd: Command) -> Response {
    let response = match (cmd) {
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
    return resonse;
}