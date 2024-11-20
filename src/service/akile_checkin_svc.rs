use std::collections::HashMap;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::util::{http_util, tg_util};

pub async fn checkin_akile(config: &crate::config::app_config::Config) {
    let res = checkin(&config).await;
    match res {
        Err(e) => {
            tg_util::send_plain_msg(config, format!("Akile 签到失败：{}", e)).await;
        },
        _ => (),
    };
}

async fn checkin(config: &crate::config::app_config::Config) -> anyhow::Result<()> {
    let token = login(config).await?;
    let username = get_user_info(&token).await?;
    let ak_coin = get_user_index(&token).await?;
    
    let mut header_map = HashMap::new();
    header_map.insert("Authorization".to_string(), token.to_string());
    let res = http_util::get("https://api.akile.io/api/v1/user/Checkin", header_map).await;
    tracing::info!("签到响应：{:?}", res);
    let res: AkileResonseBody<u32> = match res {
        Err(e) => return Err(anyhow!(format!("签到失败：{}", e))),
        Ok(res) => serde_json::from_str(&res)?,
    };
    if res.status_code != 0 {
        return Err(anyhow!(format!("签到失败：{:#?}", res.status_msg)))
    }
    tg_util::send_plain_msg(config, format!("Akile: {} 今日获得 {} AK币，总计 {} AK币", username, res.data - ak_coin, res.data)).await;

    anyhow::Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AkileResonseBody<T> {
    status_code: i32,
    status_msg: String,
    data: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LoginRequestBody {
    email: String,
    password: String,
    token: String,
    #[serde(rename = "verifyCode")]
    verify_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct LoginResonseBody {
    token: String,
}

async fn login(config: &crate::config::app_config::Config) -> anyhow::Result<String> {
    let body = LoginRequestBody {
        email: config.akile.email.clone(),
        password: config.akile.password.clone(),
        token: "".to_string(),
        verify_code: "".to_string(),
    };
    let res = http_util::post("https://api.akile.io/api/v1/user/login", serde_json::to_string(&body).unwrap()).await;
    tracing::info!("登录响应：{:?}", res);
    let res: AkileResonseBody<LoginResonseBody> = match res {
        Err(e) => return Err(anyhow!(format!("登录失败：{}", e))),
        Ok(res) => serde_json::from_str(&res)?,
    };
    if res.status_code != 0 {
        return Err(anyhow!(format!("登录失败：{}", res.status_msg)))
    }
    anyhow::Ok(res.data.token)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UserInfoResonseBody {
    username: String,
}

async fn get_user_info(token: &str) -> anyhow::Result<String> {
    let mut header_map = HashMap::new();
    header_map.insert("Authorization".to_string(), token.to_string());
    let res = http_util::get("https://api.akile.io/api/v1/user/login", header_map).await;
    tracing::info!("用户信息响应：{:?}", res);
    let res: AkileResonseBody<UserInfoResonseBody> = match res {
        Err(e) => return Err(anyhow!(format!("获取用户信息失败：{}", e))),
        Ok(res) => serde_json::from_str(&res)?,
    };
    if res.status_code != 0 {
        return Err(anyhow!(format!("获取用户信息失败：{:#?}", res.status_msg)))
    }
    anyhow::Ok(res.data.username)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UserIndexResonseBody {
    ak_coin: u32,
}

async fn get_user_index(token: &str) -> anyhow::Result<u32> {
    let mut header_map = HashMap::new();
    header_map.insert("Authorization".to_string(), token.to_string());
    let res = http_util::get("https://api.akile.io/api/v1/user/index", header_map).await;
    tracing::info!("用户索引响应：{:?}", res);
    let res: AkileResonseBody<UserIndexResonseBody> = match res {
        Err(e) => return Err(anyhow!(format!("获取用户ak币失败：{}", e))),
        Ok(res) => serde_json::from_str(&res)?,
    };
    if res.status_code != 0 {
        return Err(anyhow!(format!("获取用户ak币失败：{:#?}", res.status_msg)))
    }
    anyhow::Ok(res.data.ak_coin)
}
