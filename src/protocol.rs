use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Ping,
    Get { key: String },
    Set { key: String, value: String },
    Incr { key: String },
    IncrBy { key: String, value: i64 },
    Command,
    Error { msg: String },
    Save,
    Config,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Debug)]
pub enum Response {
    Pong,
    Get { value: String },
    OK,
    Error { msg: String },
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot;
pub struct CommandWrapper {
    pub cmd: Command,
    pub resp: oneshot::Sender<Response>,
}

pub type CommandReceiver = Receiver<CommandWrapper>;
