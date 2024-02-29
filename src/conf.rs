use serde::{Deserialize, Serialize};

pub struct LoggingConfig;

impl LoggingConfig {
    pub fn init() {
        // 创建一个输出到控制台的 appender
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    }
}

#[derive(Debug, Serialize, Deserialize)]
pub  struct RedConfig {
    #[serde(default = "default_host")]
    pub host:String,
    #[serde(default = "default_nil_string")]
    pub pwd:String,
    #[serde(default = "default_red_port")]
    pub port:u16,
    #[serde(default = "default_db")]
    pub db:u16,
}

fn default_host() -> String {
    "127.0.0.1".to_owned()
}

fn default_nil_string() -> String {
    "".to_owned()
}

fn default_red_port() -> u16 {
    6379
}

fn default_db() -> u16 {
   1
}


