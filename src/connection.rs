
use bytes::{ BytesMut};
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;
#[derive(Debug)]
pub struct Connection {
    stream: BufWriter<TcpStream>
}

impl Connection {

    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(socket)
        }
    }

    fn get_buffer() -> BytesMut {
        //TODO to tune default 4 KB
        return BytesMut::with_capacity(4 * 1024);
    }

    pub async  fn write(&mut self) -> () {
        self.stream.write_all(b"toto").await;
    }
}