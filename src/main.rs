mod config;
mod trun_error;
mod file_utils;

use std::{process::exit, collections::HashMap};

use chrono::prelude::*;
use commander::Commander;
use config::{DataConfig};
use log::*;
use trun_error::TrunResult;

fn do_repeat_check_file(config: &DataConfig, last_update_times: &mut HashMap<String, i64>) -> TrunResult<()> {
    let now = Utc::now().timestamp();
    for data in &config.all_config {
        let step = data.1.get_period_step();
        if now - last_update_times[data.0] > step {
            let info = file_utils::get_all_path(&data.0.to_string())?;
            trace!("处理{:?},目录下文件{:?}", data.0, info);
            for sub in info {
                file_utils::do_oper_log_split(&sub, &data.1)?;
            }
            *last_update_times.get_mut(data.0).unwrap() = now;
        }
    }
    Ok(())
}

fn init_log4rs() {
    use log4rs::append::console::ConsoleAppender;
    use log4rs::append::file::FileAppender;
    use log4rs::encode::pattern::PatternEncoder;
    use log4rs::config::{Appender, Config, Logger, Root};
    let stdout = ConsoleAppender::builder().build();
    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("truncate.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(Logger::builder().build("app::backend::db", LevelFilter::Trace))
        .logger(Logger::builder()
            .appender("requests")
            .additive(false)
            .build("app::requests", LevelFilter::Trace))
        .build(Root::builder().appender("stdout").appender("requests").build(LevelFilter::Trace))
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();

}

fn sub_main() -> TrunResult<()> {
    let command = Commander::new()
    .version(&env!("CARGO_PKG_VERSION").to_string())
    .usage("file_truncate")
    .usage_desc("日志切割工具")
    .option_str("-c, --config [value]", "config data ", Some("config.yaml".to_string()))
    .parse_env_or_exit()
    ;

    init_log4rs();
    trace!("分割文件程序启动");

    let config = DataConfig::read_from_file(command.get_str("c").unwrap())?;
    let mut last_update_times = HashMap::new();
    for data in &config.all_config {
        last_update_times.insert(data.0.clone(), 0i64);
    }
    loop {
        trace!("处理程序处理中");
        do_repeat_check_file(&config, &mut last_update_times)?;
        trace!("处理程序休眠中,等待下次重试");
        ::std::thread::sleep(::std::time::Duration::from_millis(10000));
    }
}

#[forever_rs::main]
fn main() {
    match sub_main() {
        Ok(()) => {
            trace!("退出程序,正常退出")
        }
        Err(_err) => 
        {
            error!("退出程序,异常退出,原因{:?}", _err);
            exit(1)
        }
    }
}
