mod config;
mod trun_error;
mod file_utils;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::{self, File};
use config::{DataConfig, OneConfig};
use trun_error::TrunResult;
use chrono::prelude::*;

fn calc_path(file: String, config: &OneConfig, idx: u64) -> String {
    let dateext = config.dateext.as_ref().unwrap();
    println!("dateext = {:?}", dateext);
    if dateext.len() == 0 {
        file + "." + &idx.to_string()
    } else {
        let utc: DateTime<Utc> = Utc::now();
        println!("path111111111111 = {:?}", file.to_owned() + "." + &utc.format(dateext).to_string() + "." + &idx.to_string());
        file + "." + &utc.format(dateext).to_string() + "." + &idx.to_string()
    }
}
fn do_oper_log_split(path: &(String, String), config: &OneConfig) -> TrunResult<()> {
    let real_path = file_utils::real_path(&path);
    let meta = fs::metadata(&real_path)?;
    if !meta.is_file() {
        return Ok(());
    }

    let rotate = config.rotate.unwrap_or(0);
    let trun_size = config.truncate_size();
    if trun_size == 0 {
        return Ok(())
    }

    if meta.len() < trun_size {
        return Ok(());
    }

    let step =  (meta.len() as f64 / trun_size as f64).ceil() as u64;
    println!("meta len = {:?} step = {:?}", meta.len(), step);
    let mut start_step = 0;
    let mut need_offset = step - 1;
    if rotate + 1 < step {
        start_step = step - rotate - 1;
        need_offset = 0;
    }


    if need_offset != 0 {
        for idx in 0 .. rotate {
            let now_path = file_utils::real_path(&(path.0.clone(), calc_path(path.1.clone(), config, idx)));
            let dest_path = file_utils::real_path(&(path.0.clone(), calc_path(path.1.clone(), config, idx + need_offset)));
            let _ = fs::rename(now_path, dest_path);
        }
    }
    
    let mut one_kb = [0u8; 1024];
    let mut log_file = OpenOptions::new().read(true).write(true).open(&real_path)?;
    for idx in start_step .. step - 1 {
        let dest_path = file_utils::real_path(&(path.0.clone(), calc_path(path.1.clone(), config, idx - start_step)));
        let mut dest_file = File::create(dest_path)?;
        log_file.seek(SeekFrom::Start(idx * trun_size))?;
        let mut read_byte = trun_size as i64;
        while read_byte > 0 {
            match log_file.read(&mut one_kb) {
                Ok(size) => {
                    let _ = dest_file.write(&one_kb[..size]);
                    read_byte -= size as i64;
                    if size < 1024 {
                        break;
                    }
                }
                Err(_err) => {
                    break;
                }
            };
        }
    }

    log_file.seek(SeekFrom::Start(0))?;
    let mut clone_log_file = OpenOptions::new().read(true).open(&real_path)?;
    clone_log_file.seek(SeekFrom::Start((step - 1) * trun_size))?;
    let mut real_size = 0 as i64;
    loop {
        match clone_log_file.read(&mut one_kb) {
            Ok(size) => {
                let _ = log_file.write(&one_kb[..size]);
                real_size += size as i64;
                if size < 1024 {
                    break;
                }
            }
            Err(_err) => {
                break;
            }
        };
    }
    log_file.set_len(real_size as u64)?;
    log_file.flush()?;

    return Ok(())
    

}

fn main() -> TrunResult<()> {
    let config = DataConfig::read_from_file("config.yaml".to_string())?;
    for data in config.all_config {
        let info = file_utils::get_all_path(&data.0.to_string())?;
        println!("info {:?}", info);
        for sub in info {
            do_oper_log_split(&sub, &data.1)?;
        }
    }
    Ok(())
}
