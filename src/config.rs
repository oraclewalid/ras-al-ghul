use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct Config  {
    server  : Option<ServerConfig>,
    storage : Option<StorageConfig>,
}

#[derive(Debug, Deserialize, Clone)]
struct ServerConfig {
    bind    : Option<String>,
    port    : Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
struct StorageConfig {
    snapshot : Option<bool>,
    db_file_name: Option<String>,
    save        : Option<u64>,
}


pub fn get_config() {
    let toml_str = r#"
    [server]
    ip = "127.0.0.1"
    port = 80
    [storage]
    snapshot = true
    db_file_name = "/tmp/ras/dg.ser"
    save = 100
"#;
    
    let decoded: ServerConfig = toml::from_str(toml_str).unwrap();
}

#[test]
fn parse_config_in_toml_format() {

    let toml_str = r#"
        [server]
        bind = "127.0.0.1"
        port = 80
        [storage]
        snapshot = true
        db_file_name = "/tmp/ras/ras-al-ghul.db"
        save = 1000
    "#;
    let config: Config= toml::from_str(toml_str).unwrap();

   assert_eq!( config.clone().server.unwrap().bind.unwrap(), "127.0.0.1");
   assert_eq!( config.clone().server.unwrap().port.unwrap(), 80);
   assert_eq!( config.clone().storage.unwrap().snapshot.unwrap(), true);
   assert_eq!( config.clone().storage.unwrap().db_file_name.unwrap(), "/tmp/ras/ras-al-ghul.db");
   assert_eq!( config.clone().storage.unwrap().save.unwrap(), 1000);
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
tcp-keepalive 300

SNAPSHOTTING  
save 900 1
dbfilename dump.rdb

logfile 
*/