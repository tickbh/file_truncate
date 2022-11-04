use std::ops::Range;
use std::fs::{self, metadata, read_dir, File};
use regex::Regex;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;
use crate::config::{OneConfig};
use crate::trun_error::TrunResult;
use chrono::prelude::*;
use crate::trun_error::TrunError;

pub fn get_real_path(path: &(String, String)) -> String {
    return path.0.to_string() + "/" + &path.1;
}

pub fn split_path(path: &String) -> (String, String) {
    let path = path.replace("\\", "/");
    let mut paths: Vec<&str> = path.split("/").collect();
    if paths.len() == 1 {
        return (String::new(), paths[0].to_string());
    } else {
        
        let last = paths.remove(paths.len() - 1);
        return (paths.join("/"), last.to_string());
    }
}

pub fn get_all_path(root_path: &String) -> Result<Vec<(String, String)>, TrunError> {
    match fs::metadata(root_path) {
        Ok(file) => {
            if file.is_file() {
                return Ok(vec![split_path(root_path)]);
            }
        }
        Err(_) => {}
    }
    let paths: Vec<&str> = root_path.split("/").collect();
    let mut idx = 0usize;
    let mut need_match = false;
    for path in &paths {
        if !path.contains("*") {
            idx += 1;
        } else {
            need_match = true;
            break;
        }
    }
    let first_path: String = if idx == 0 {
        String::new()
    } else {
        paths[0..idx].join("/")
    };
    let re = if !need_match {
        Regex::new(r"\**").unwrap()
    } else {
        let info = "^".to_string() + &root_path.replace("*", "([\\w\\.-_]+)") + "$";
        match Regex::new(&info) {
            Ok(reg) => {
                reg
            }
            Err(_err) => {
                return Ok(vec![]);
            }
        }
    };

    let export_re = Regex::new(r"^*\.\d+$").unwrap();
    let mut path_list = vec![String::from(first_path)];
    let mut result_list = vec![];
    let mut start_index = 0;
    loop {
        let list_len = path_list.len();
        for index in start_index..path_list.len() {
            let mut path = path_list[index].to_string();
            if metadata(&path)?.is_dir() {
                for child_dir in read_dir(&path)? {
                    path_list.push(String::from(child_dir?.path().as_os_str().to_str().expect("")));
                }
            } else {
                path = path.replace("\\", "/");
                if re.is_match(&path) && !export_re.is_match(&path) {
                    result_list.push(split_path(&path));
                }
            }
        }
        if list_len == start_index { break; }
        start_index = list_len;
    }
    Ok(result_list)
}

fn calc_path(file: String, config: &OneConfig, idx: u64) -> String {
    let dateext = config.dateext.as_ref().unwrap();
    if dateext.len() == 0 {
        file + "." + &idx.to_string()
    } else {
        let utc: DateTime<Utc> = Utc::now();
        file + "." + &utc.format(&dateext).to_string() + "." + &idx.to_string()
    }
}
pub fn do_oper_log_split(path: &(String, String), config: &OneConfig) -> TrunResult<()> {
    let real_path = get_real_path(&path);
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
    let mut start_step = 0;
    let mut need_offset = step - 1;
    if rotate + 1 < step {
        start_step = step - rotate - 1;
        need_offset = 0;
    }


    if need_offset != 0 {
        for idx in (0 .. rotate).rev() {
            let now_path = get_real_path(&(path.0.clone(), calc_path(path.1.clone(), config, idx)));
            let dest_path = get_real_path(&(path.0.clone(), calc_path(path.1.clone(), config, idx + need_offset)));
            println!("rename now_path === {:?} dest_path = {:?}", now_path, dest_path);
            let _ = fs::rename(now_path, dest_path);
            
        }
    }
    
    let mut one_kb = [0u8; 1024];
    let mut log_file = OpenOptions::new().read(true).write(true).open(&real_path)?;
    for idx in start_step .. step - 1 {
        let dest_path = get_real_path(&(path.0.clone(), calc_path(path.1.clone(), config, idx - start_step)));
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