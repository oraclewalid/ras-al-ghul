use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

use crate::protocol::*;
use crate::config::Config;
use tokio::time::sleep;

use std::time::Duration;

pub async fn start_persistance_cron(tx: Sender<CommandWrapper>, conf: Config) -> Response {
    if conf.storage.snapshot {
        loop {
            let (resp_tx, resp_rx) = oneshot::channel::<Response>();
    
            let send = tx.send(CommandWrapper{ cmd : Command::Save, resp : resp_tx}).await;
        
            let res = resp_rx.await.unwrap_or(Response::Error{msg : "Unknown Error".into()});
            println!("{}",res);
            sleep(Duration::from_millis(conf.storage.save.unwrap_or(1000))).await;
        }
        
    }
    Response::OK

}
