use serde_derive::Deserialize;

use std::fs::File;
use std::io::Read;
use std::io;

#[derive(Debug, Deserialize, Clone)]
pub struct Config  {
    pub server  : Option<ServerConfig>,
    pub storage : Option<StorageConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    bind    : Option<String>,
    port    : Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StorageConfig {
    snapshot : Option<bool>,
    db_file_name: Option<String>,
    save        : Option<u64>,
}


pub fn get_config(filename: String) -> Result<Config, io::Error> {

    let file_content = get_file_as_str(&filename);
    
    match file_content {
        Ok(file_content)=> parse_config(file_content),
        Err(e)              => Err(e),
    }
}


fn get_file_as_str(filename: &String) -> Result<String, io::Error> {
    let mut f = File::open(filename)?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}
fn parse_config(config: String) -> Result<Config, io::Error> {
    
     return toml::from_str::<Config>(&config.as_str()).map_err(|e| io::Error::from(io::ErrorKind::InvalidInput));
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

    let config: Config=  parse_config(toml_str.into()).unwrap();

   assert_eq!( config.clone().server.unwrap().bind.unwrap(), "127.0.0.1");
   assert_eq!( config.clone().server.unwrap().port.unwrap(), 80);
   assert_eq!( config.clone().storage.unwrap().snapshot.unwrap(), true);
   assert_eq!( config.clone().storage.unwrap().db_file_name.unwrap(), "/tmp/ras/ras-al-ghul.db");
   assert_eq!( config.clone().storage.unwrap().save.unwrap(), 1000);
}

