mod config;
mod trun_error;
mod file_utils;

use std::{process::exit, collections::HashMap};

use chrono::prelude::*;
use commander::Commander;
use config::{DataConfig};
use trun_error::TrunResult;

fn do_repeat_check_file(config: &DataConfig, last_update_times: &mut HashMap<String, i64>) -> TrunResult<()> {
    let now = Utc::now().timestamp();
    for data in &config.all_config {
        let step = data.1.get_period_step();
        if now - last_update_times[data.0] > step {
            let info = file_utils::get_all_path(&data.0.to_string())?;
            println!("info {:?}", info);
            for sub in info {
                file_utils::do_oper_log_split(&sub, &data.1)?;
            }
            *last_update_times.get_mut(data.0).unwrap() = now;
        }
    }
    Ok(())
}

fn sub_main() -> TrunResult<()> {
    let command = Commander::new()
    .version(&env!("CARGO_PKG_VERSION").to_string())
    .usage("file_truncate")
    .usage_desc("日志切割工具")
    .option_str("-c, --config [value]", "config data ", Some("config.yaml".to_string()))
    .parse_env_or_exit()
    ;

    let config = DataConfig::read_from_file(command.get_str("c").unwrap())?;
    let mut last_update_times = HashMap::new();
    for data in &config.all_config {
        last_update_times.insert(data.0.clone(), 0i64);
    }
    loop {
        do_repeat_check_file(&config, &mut last_update_times)?;
        ::std::thread::sleep(::std::time::Duration::from_millis(10000));
    }
    
    Ok(())
}

#[forever_rs::main]
fn main() {
    println!("ok!!");
    match sub_main() {
        Ok(()) => {
            println!("ok!!")
        }
        Err(_err) => exit(1)
    }
}
