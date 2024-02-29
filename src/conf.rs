use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub  struct RedConfig {
    pub host:Option<String>,
    pub pwd:Option<String>,
    pub port:Option<u16>,
    pub db:Option<u16>,
}

impl Default for RedConfig {
    fn default() -> Self {
        RedConfig {
            host:Some(String::from("127.0.0.1")),
            pwd: Some(String::from("")),
            port: Some(6379),
            db: Some(0),
        }
    }
}

impl  RedConfig {
    pub fn new(host:Option<String>,pwd:Option<String>,port:u16,db:u16)-> Self {
        RedConfig {
            host:Some(String::from(host)),
            pwd: Some(String::from(pwd)),
            port: Some(port),
            db: Some(db),
        }

    }
}