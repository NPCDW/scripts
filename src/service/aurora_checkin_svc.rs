use std::collections::HashMap;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::util::{http_util, tg_util};

pub async fn checkin_aurora(config: &crate::config::app_config::Config) {
    let res = checkin(&config).await;
    match res {
        Err(e) => {
            tg_util::send_plain_msg(config, format!("Aurora 签到失败：{}", e)).await;
        },
        _ => (),
    };
}

async fn checkin(config: &crate::config::app_config::Config) -> anyhow::Result<()> {
    let mut header_map = HashMap::new();
    header_map.insert("Authorization".to_string(), config.aurora.authorization.clone());
    let res = http_util::get("https://server.auroramedia.me/aurora/v1/user/checkin", header_map).await;
    tracing::info!("签到响应：{:?}", res);
    let res: AuroraResonseBody<AuroraCheckinResonseBody> = match res {
        Err(e) => return Err(anyhow!(e)),
        Ok(res) => serde_json::from_str(&res)?,
    };
    if res.code != 200 {
        return Err(anyhow!(res.message))
    }
    let data = res.data.unwrap();
    tg_util::send_plain_msg(config, format!("Aurora: 今日获得 {} 灵石，总计 {} 灵石", data.add_points, data.points)).await;

    anyhow::Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AuroraResonseBody<T> {
    code: i32,
    message: String,
    data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AuroraCheckinResonseBody {
    #[serde(rename = "addPoints")]
    add_points: i32,
    points: i32,
}
