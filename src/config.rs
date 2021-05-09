use serde_derive::Deserialize;
#[derive(Debug, Deserialize)]
struct ServerConfig {
    ip: Option<String>,
    port: Option<u8>,
}

pub fn get_config() {
    let toml_str = r#"
    global_string = "test"
    global_integer = 5
    [server]
    ip = "127.0.0.1"
    port = 80
    [[peers]]
    ip = "127.0.0.1"
    port = 8080
    [[peers]]
    ip = "127.0.0.1"
"#;

    let decoded: ServerConfig = toml::from_str(toml_str).unwrap();
}
/*
use std::fs::File;
use std::io::Read;

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}
*/