use serde_derive::Deserialize;

use std::fs::File;
use std::io::Read;
use std::io;

#[derive(Debug, Deserialize, Clone)]
pub struct Config  {
    pub server      : ServerConfig,
    pub snapshot    : SnapshotConfig,
    pub storage     : StorageConfig,
}

impl Default for Config {
    fn default() -> Config {
        Config { server: ServerConfig::default(), snapshot: SnapshotConfig::default(), storage: StorageConfig::default() }
   }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    bind    : String,
    port    : u64,
}

impl Default for ServerConfig {
    fn default() -> ServerConfig {
        ServerConfig { bind: "0.0.0.0".into(), port: 6543 }
   }
}

impl ServerConfig {
    pub fn to_server_with_port(&self) -> String {
        format!("{}:{}", self.bind, self.port)
    }
}


#[derive(Debug, Deserialize, Clone)]
pub struct SnapshotConfig {
    pub snapshot    : bool,
    pub db_file_name: Option<String>,
    pub save        : Option<u64>,
}

impl Default for SnapshotConfig {
    fn default() -> SnapshotConfig {
        SnapshotConfig { snapshot: false, db_file_name: Some("/tmp/ras-al-ghul.db".into()), save: Some(10000) }
   }
}

#[derive(Debug, Deserialize, Clone)]
pub struct StorageConfig {
    pub backend   : StorageBackend,
    pub path      : Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]

pub enum StorageBackend {
    Inmemory,
    Rocksdb
}

impl Default for StorageConfig {
    fn default() -> StorageConfig {
        StorageConfig { backend: StorageBackend::Inmemory, path: None}
   }
}


pub fn get_config(filename: Option<String>) -> Config {
    
    if filename.is_none(){
        return Config::default();
    }

    let file_content = get_file_as_str(&filename.unwrap());
    
    match file_content {
        Ok(file_content)=> parse_config(file_content).unwrap_or(Config::default()),
        _                      => Config::default(),
    }
}


fn get_file_as_str(filename: &String) -> Result<String, io::Error> {
    let mut f = File::open(filename)?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}
fn parse_config(config: String) -> Result<Config, toml::de::Error> {
    
     return toml::from_str::<Config>(&config.as_str());
}

#[test]
fn parse_config_in_toml_format() {

    let toml_str = r#"
        [server]
        bind = "127.0.0.1"
        port = 80
        [snapshot]
        snapshot = true
        db_file_name = "/tmp/ras/ras-al-ghul.db"
        save = 1000
        [storage]
        backend = "Rocksdb"
        path = "/tmp/rocksdb"
    "#;
    let config: Config=  parse_config(toml_str.into()).unwrap();

   assert_eq!( config.clone().server.bind, "127.0.0.1");
   assert_eq!( config.clone().server.port, 80);
   assert_eq!( config.clone().snapshot.snapshot, true);
   assert_eq!( config.clone().snapshot.db_file_name.unwrap(), "/tmp/ras/ras-al-ghul.db");
   assert_eq!( config.clone().snapshot.save.unwrap(), 1000);
   assert_eq!( config.clone().storage.backend, StorageBackend::Rocksdb);
   assert_eq!( config.clone().storage.path.unwrap(), "/tmp/rocksdb");
}

#[test]
fn return_default_config_if_config_file_dont_exist() {


    let config: Config=  get_config(Some("/path/to/file.cfg".into()));

   assert_eq!( config.clone().server.bind, "0.0.0.0");
   assert_eq!( config.clone().server.port, 6543);
   assert_eq!( config.clone().snapshot.snapshot, false);
   assert_eq!( config.clone().snapshot.db_file_name.unwrap(), "/tmp/ras-al-ghul.db");
   assert_eq!( config.clone().snapshot.save.unwrap(), 10000);
   assert_eq!( config.clone().storage.backend, StorageBackend::Inmemory);
   assert_eq!( config.clone().storage.path, None);
}

#[test]
fn return_default_config_if_no_config_file_is_provided() {


    let config: Config=  get_config(Some("/path/to/file.cfg".into()));

   assert_eq!( config.clone().server.bind, "0.0.0.0");
   assert_eq!( config.clone().server.port, 6543);
   assert_eq!( config.clone().snapshot.snapshot, false);
   assert_eq!( config.clone().snapshot.db_file_name.unwrap(), "/tmp/ras-al-ghul.db");
   assert_eq!( config.clone().snapshot.save.unwrap(), 10000);
}

