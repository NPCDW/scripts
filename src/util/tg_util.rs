use serde::{Deserialize, Serialize};

use crate::util::http_util;

pub async fn send_plain_msg(config: &crate::config::app_config::Config, text: String) {
    send_msg(config, text, None).await;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SendMsgBody {
    chat_id: String,
    message_thread_id: u64,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<String>,
}

pub async fn send_msg(config: &crate::config::app_config::Config, text: String, parse_mode: Option<String>) {
    if let Some(tg) = &config.tg {
        let url = format!("https://api.telegram.org/bot{}/sendMessage", tg.bot_token);
        let body = SendMsgBody {
            chat_id: tg.chat_id.clone(),
            message_thread_id: tg.topic_id.clone(),
            text: text,
            parse_mode: parse_mode,
        };
        let body = serde_json::to_string(&body).unwrap();
        tracing::debug!("tg 发送消息 body: {}", &body);
        match http_util::post(&url, body).await {
            Ok(_) => tracing::info!("tg 消息发送成功"),
            Err(e) => tracing::error!("tg 消息发送失败: {}", e),
        }
    }
}
