mod config;
mod trun_error;
mod file_utils;

use std::fs;
use config::{DataConfig, OneConfig};
use trun_error::TrunResult;

fn do_oper_log_split(path: (String, String), config: OneConfig) -> TrunResult<()> {
    let path = file_utils::real_path(&path);
    let meta = fs::metadata(path)?;
    if !meta.is_file() {
        return Ok(());
    }

    let rotate = config.rotate.unwrap_or(0);
    if rotate == 0 {
        return Ok(());
    }

    let trun_size = config.truncate_size();
    if trun_size == 0 {
        return Ok(())
    }

    if meta.len() < trun_size {
        return Ok(());
    }

    let step =  (meta.len() as f64 / trun_size as f64).ceil() as u64;
    let mut start_step = 0;
    if rotate + 1 < step {
        start_step = step - rotate - 1;
    }
    


    return Ok(())
    

    // meta.len()
}

fn main() -> TrunResult<()> {
    let info = file_utils::get_all_path(&"D:/game/poker_server/scripts/*/daemons/*/*.lua".to_string())?;
    println!("info {:?}", info);
    let config = DataConfig::read_from_file("config.yaml".to_string());
    println!("Hello, world! {:?}", config);
    Ok(())
}
