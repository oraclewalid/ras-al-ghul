
use serde_cbor::Error;

use crate::config::Config;
use crate::protocol::*;
use crate::database::*;
use rand::prelude::*;


pub async fn start_memory_manager(mut rx: CommandReceiver, conf: Config) {

    let mut db = get_storage_backend(conf.storage.clone());

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
                if key.eq("__rand_int__") {
                    let mut rng = rand::thread_rng();
                    let random_int: u8 = random();
                    Response::Get{value: random_int.to_string()}
                } else {
                    
                    let value = db.get(&key.clone());
                    match value {
                        Some(value ) => Response::Get{value: value.clone()},
                        None => Response::Error{msg : format!("The key {} not found", key)}
                    }
                }

            },
            Command::Incrby{key, value} => {
                let db_value = db.get(&key)
                        .or(Some(String::from("0")))
                        .and_then(|value | value.parse::<i64>().ok());
                match db_value {
                    Some(db_value) =>{
                        let new_value = db_value + value;
                        db.set(key, new_value.to_string());
                        Response::Get{value: new_value.to_string()}
                    },
                    None  =>  Response::Error{msg : "ERR value is not an integer or out of range".into()}
                }

            },
            Command::Incr{key} => {
                let db_value = db.get(&key)
                    .or(Some(String::from("0")))
                    .and_then(|value | value.parse::<i64>().ok());
                match db_value {
                    Some(db_value) => {
                        let new_value = db_value + 1;
                        db.set(key, new_value.to_string());
                        Response::Get{value: new_value.to_string()}
                    },
                    None  =>  Response::Error{msg : "ERR value is not an integer or out of range".into()}
                }

            },
            Command::Save => {
                if conf.snapshot.snapshot == true {
                    let persistance =  Err("");  //db.persist(conf.snapshot.clone().db_file_name.unwrap());
                    match persistance {
                        Ok(()) =>  {
                            println!("DB persisted on {}", conf.snapshot.clone().db_file_name.unwrap());
                            Response::OK
                        },
                        _  =>  Response::Error{msg : "ERROR, the database was not persisted".into()},
                    }
                }
                else {
                    Response::Error{msg : "ERROR, snapshot of DB is not activated".into()}
                }

            },
            _ => Response::Error{msg : "Unknown command".into()}
        };

        let sent_response = rx.send(response);
        match sent_response {
            Result::Ok(_) => println!("Response sent"),
            Result::Err(error) => println!("Error in message sent {}", error)
        }
    }
}

fn load_db_or_create_new(conf: Config) -> InMemoryDatabase {
    if conf.snapshot.snapshot == true {
        println!("Loading snapshot from {}", conf.snapshot.clone().db_file_name.unwrap());
        let db_result = InMemoryDatabase::load(conf.snapshot.clone().db_file_name.unwrap());
        match db_result {
            Ok(db) =>  db,
            _  =>  panic!("Can't deserialize DB!"),
        }
    }
    else {
        InMemoryDatabase::new()
    }
    
}