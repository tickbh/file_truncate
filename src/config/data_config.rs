
use std::io::{self, Read};
use std::fs::File;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use super::TrunResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct DataConfig {
    // 轮转数量
    #[serde(default = "default_rotate")]
    pub rotate: i32,
    // 周期
    #[serde(default = "default_period")]
    pub period: String,
    // 转存储大小
    #[serde(default = "default_size")]
    pub size: String,

    pub all_config: HashMap<String, OneConfig>,
}

fn default_rotate() -> i32 {
    0
}

fn default_period() -> String {
    "daily".to_owned()
}

fn default_size() -> String {
    "100M".to_owned()
}


#[derive(Serialize, Deserialize, Debug)]
pub struct OneConfig {
    // 轮转数量
    pub rotate: Option<i32>,
    // 周期
    pub period: Option<String>,
    // 转存储大小
    pub size: Option<String>,
}

impl Default for DataConfig {
    fn default() -> Self { 
        DataConfig {
            rotate: 0,
            period: "daily".to_string(),
            size: "100M".to_string(),
            all_config: HashMap::new(),
        }
    }
}

impl DataConfig {
    pub fn read_from_file(file: String) -> TrunResult<DataConfig>  {
        let mut f = File::open(file)?;
        let mut buffer = Vec::new();
        // read the whole file
        f.read_to_end(&mut buffer)?;
        let file_data = String::from_utf8_lossy(&buffer);
        let field: DataConfig = serde_yaml::from_str(&file_data)?;
        return Ok(field);
    }
}