use std::fs::File;
use std::io::Read;

mod conf;slint::include_modules!();
use conf:: RedConfig;

// slint宏，创建 UI

fn main() {
    let mut f = File::open("config.yml").expect("无法读取配置");
    let mut scrape_config:RedConfig = serde_yaml::from_reader(f).expect("Could not read values.");
    println!("{:?}", scrape_config);
    MainWindow::new().unwrap().run().unwrap();
}