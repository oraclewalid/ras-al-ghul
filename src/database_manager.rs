
use crate::protocol::*;
use crate::database;

pub async fn start_memory_manager(mut rx: CommandReceiver) {

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
