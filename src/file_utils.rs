use std::fs::{self, metadata, read_dir, File};
use regex::Regex;

use crate::trun_error::TrunError;

pub fn real_path(path: &(String, String)) -> String {
    return path.0.to_string() + "/" + &path.1;
}

pub fn split_path(path: &String) -> (String, String) {
    let path = path.replace("\\", "/");
    let mut paths: Vec<&str> = path.split("/").collect();
    if paths.len() == 1 {
        return (String::new(), paths[0].to_string());
    } else {
        
        let last = paths.remove(paths.len() - 1);
        println!("laslt = {:?} paths = {:?}", last, paths);
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
        let info = root_path.replace("*", "([\\w\\._]+)");
        match Regex::new(&info) {
            Ok(reg) => {
                reg
            }
            Err(_err) => {
                print!("err = {:?}", _err);
                return Ok(vec![]);
            }
        }
        
    };

    println!("first = {:?}",first_path);
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
                println!("path == {:?}", path);
                if re.is_match(&path)  {
                    result_list.push(split_path(&path));
                }
            }
        }
        if list_len == start_index { break; }
        start_index = list_len;
    }
    Ok(result_list)
}

fn all_path(root_path: &String) -> Result<Vec<String>, TrunError> {
    let mut path_list = vec![String::from(root_path)];
    let mut start_index = 0;
    loop {
        let list_len = path_list.len();
        for index in start_index..path_list.len() {
            let path = &path_list[index];
            if metadata(path)?.is_dir() {
                for child_dir in read_dir(&path)? {
                    path_list.push(String::from(child_dir?.path().as_os_str().to_str().expect("")));
                }
            }
        }
        if list_len == start_index { break; }
        start_index = list_len;
    }
    return Ok(path_list);
}