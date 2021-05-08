use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Ping,
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
    },
    Command,
    Error{
        msg: String
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Debug)]
pub enum Response {
    Pong,
    Get {
        value: String,
    },
    OK,
    Error{
        msg: String
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}