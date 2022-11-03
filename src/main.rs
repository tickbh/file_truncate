mod config;
use serde_yaml;
use std::io;
use config::{DataConfig, TrunResult};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() -> TrunResult<()>{
    let config = DataConfig::read_from_file("config.yaml".to_string());
    println!("Hello, world! {:?}", config);
    Ok(())
}
