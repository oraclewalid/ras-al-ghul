use resp::{encode, encode_slice, Decoder, Value};
use std::io::{BufReader, Error, Read};

use crate::Command;
use crate::Response;

fn parse_resp(bytes: &[u8]) -> Result<Value, Error> {
    let mut decoder = Decoder::new(BufReader::new(bytes));
    return decoder.decode();
}

fn map_resp_to_cmd(resp_value: Value) -> Command {
    match resp_value {
        Value::Array(resp_vec) => map_values_to_cmd(resp_vec),
        _ => Command::Error { msg: "TODO".into() },
    }
}

fn map_values_to_cmd(resp_value: Vec<Value>) -> Command {
    match &resp_value[..] {
        [Value::Bulk(cmd), Value::Bulk(key), Value::Bulk(value), Value::Bulk(ttl)]
            if cmd.to_lowercase() == SET =>
        {
            Command::Set {
                key: key.clone(),
                value: value.clone(),
            }
        }
        [Value::Bulk(cmd), Value::Bulk(key), Value::Bulk(value)] if cmd.to_lowercase() == SET => {
            Command::Set {
                key: key.clone(),
                value: value.clone(),
            }
        }
        [Value::Bulk(cmd), Value::Bulk(key)] if cmd.to_lowercase() == GET => {
            Command::Get { key: key.clone() }
        }
        [Value::Bulk(cmd), Value::Bulk(key)] if cmd.to_lowercase() == INCR => {
            Command::Incr { key: key.clone() }
        }
        [Value::Bulk(cmd), Value::Bulk(key), Value::Bulk(value)]
            if cmd.to_lowercase() == INCRBY =>
        {
            let value = value.parse::<i64>();
            match value {
                Ok(value) => Command::Incrby {
                    key: key.clone(),
                    value: value.clone(),
                },
                _ => Command::Error {
                    msg: "ERR value is not an integer or out of range".into(),
                },
            }
        }
        [Value::Bulk(cmd)] if cmd.to_lowercase() == PING => Command::Ping,
        [Value::Bulk(cmd)] if cmd.to_lowercase() == COMMAND => Command::Command,
        [Value::Bulk(cmd)] if cmd.to_lowercase() == SAVE => Command::Save,
        [Value::Bulk(cmd)] if cmd.to_lowercase() == CONFIG => Command::Config,
        value => {
            dbg!(value);
            Command::Error {
                msg: UNKNOWN_ERROR.into(),
            }
        }
    }
}

pub fn parse_and_map_to_command(bytes: &[u8]) -> Command {
    parse_resp(bytes)
        .map(|value| map_resp_to_cmd(value))
        .unwrap_or(Command::Error { msg: "".into() })
}

pub fn map_response_to_resp(response: Response) -> Value {
    match response {
        Response::Get { value } => Value::String(value),
        Response::OK => Value::String(OK.into()),
        Response::Pong => Value::String(PONG.into()),
        Response::Error { msg } => Value::NullArray,
    }
}

const SET: &str = "set";
const GET: &str = "get";
const INCRBY: &str = "incrby";
const INCR: &str = "incr";
const PING: &str = "ping";
const PONG: &str = "pong";
const OK: &str = "ok";
const COMMAND: &str = "command";
const UNKNOWN_ERROR: &str = "Unknown error";
const SAVE: &str = "save";
const CONFIG: &str = "config";

#[test]
fn parse_and_map_set_a_1() {
    let bcmd = "*3\r\n$3\r\nSET\r\n$1\r\na\r\n$1\r\n1\r\n"
        .to_string()
        .into_bytes();

    let cmd = parse_and_map_to_command(&bcmd);

    assert_eq!(
        cmd,
        Command::Set {
            key: "a".into(),
            value: "1".into()
        }
    );
}

#[test]
fn parse_and_map_set_a_1_with_ttl() {
    let bcmd = "*4\r\n$3\r\nSET\r\n$1\r\na\r\n$1\r\n1\r\n$1\r\n1\r\n"
        .to_string()
        .into_bytes();

    let cmd = parse_and_map_to_command(&bcmd);

    assert_eq!(
        cmd,
        Command::Set {
            key: "a".into(),
            value: "1".into()
        }
    );
}

#[test]
fn parse_and_map_get_a() {
    let bcmd = "*2\r\n$3\r\nGET\r\n$1\r\na\r\n".to_string().into_bytes();

    let cmd = parse_and_map_to_command(&bcmd);

    assert_eq!(cmd, Command::Get { key: "a".into() });
}

#[test]
fn set_is_correctly_decoded() {
    let data = encode_slice(&["SET", "a", "1"]);

    let result = parse_resp(&data);

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        Value::Array(vec![
            Value::Bulk("SET".to_string()),
            Value::Bulk("a".to_string()),
            Value::Bulk("1".to_string())
        ])
    )
}
#[test]
fn get_is_correctly_decoded() {
    let data = encode_slice(&["get", "a"]);

    let result = parse_resp(&data);

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        Value::Array(vec![
            Value::Bulk("get".to_string()),
            Value::Bulk("a".to_string())
        ])
    )
}
