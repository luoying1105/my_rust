use std::fs::File;
mod conf;slint::include_modules!();
use conf:: RedConfig;
use conf:: LoggingConfig;
use log::{debug, info};
use std::io::BufReader;

fn main() {
     // 初始化日志配置
    LoggingConfig::init();
    debug!("booting up");
    let file = File::open("config.json").unwrap();
    let reader = BufReader::new(file);
    let config: RedConfig = serde_json::from_reader(reader).unwrap();
    info!("red setting is {}:{}", config.host,config.port);
    MainWindow::new().unwrap().run().unwrap();


}