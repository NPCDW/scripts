use serde_inline_default::serde_inline_default;

use serde::{Serialize, Deserialize};
use crate::util::file_util;

#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuroraConfig {
    pub authorization: String,
}

#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TgConfig {
    pub bot_token: String,
    pub chat_id: String,
    #[serde_inline_default(0)]
    pub topic_id: u64,
}

#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde_inline_default("info".to_string())]
    pub log_level: String,
    pub aurora: AuroraConfig,
    pub tg: Option<TgConfig>
}

const CONFIG_FILE_NAME: &'static str = "config/config.json";

pub fn get_config() -> Config {
    let current_dir = file_util::get_current_dir();
    let filepath = current_dir.join(CONFIG_FILE_NAME);
    if !filepath.exists() {
        panic!("没有在工作目录 {:?} 找到 {:?}", current_dir, CONFIG_FILE_NAME);
    }
    let buf = file_util::read_file(&filepath).unwrap_or_else(|e| {
        panic!("读取配置文件失败: {}, {:?}", &filepath.display() ,e);
    });
    let config: Config = serde_json::from_str(&buf).unwrap_or_else(|e| {
        panic!("配置文件 {} 可能不是 json 格式: {:?}", &filepath.display(), e);
    });
    config
}
