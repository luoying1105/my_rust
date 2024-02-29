use std::fs::File;
mod conf;
mod red;slint::include_modules!();
use conf:: RedConfig;
use conf:: LoggingConfig;
use log::{debug, info};
use std::io::BufReader;
use red::RedisClient;
use red::RedList;
fn main() {
     // 初始化日志配置
    LoggingConfig::init();
    debug!("booting up");
    let file = File::open("config.json").unwrap();
    let reader = BufReader::new(file);
    let config: RedConfig = serde_json::from_reader(reader).unwrap();
    info!("red setting is {}:{}", config.host,config.port);
    let mut red_cache:RedisClient = RedisClient::new(config.host, config.pwd, config.port, config.db);
    red_cache.set_sting_value("ping","pong");
    let mut red_list:RedList = RedList::new();
    let _ = red_list.append(&mut red_cache, "hello","hello1");
    let _ =red_list.append(&mut red_cache, "hello","hello2");
    let _ = red_list.append(&mut red_cache, "hello","hello3");
    let res = red_list.pop(&mut red_cache, "hello");
    info!("reding res is {:?}",res);

    MainWindow::new().unwrap().run().unwrap();

}