use chrono::{DateTime, NaiveDate};

use crate::{
    error::{AppError, AppResult},
    storage::supabase::SupabaseClient,
};

pub async fn sync_weight(payload: serde_json::Value) -> AppResult<()> {
    let repository = SupabaseClient::from_config_file()?;
    let Some(data) = payload.get("data") else {
        return AppResult::Err(AppError::Message("data is empty".to_string()));
    };
    let Some(metrics) = data.get("metrics") else {
        return AppResult::Err(AppError::Message("metrics is empty".to_string()));
    };
    let Some(metrics) = metrics.as_array() else {
        return AppResult::Err(AppError::Message("metrics is empty".to_string()));
    };
    for metric in metrics {
        let name = metric["name"].as_str().unwrap();
        if name != "weight_body_mass" {
            continue;
        }
        let data = metric["data"].as_array();
        let Some(data) = data else {
            continue;
        };
        for item in data {
            // 1. 先解析为带偏移的 DateTime
            let dt =
                DateTime::parse_from_str(&item["date"].as_str().unwrap(), "%Y-%m-%d %H:%M:%S %z")
                    .unwrap();
            // 2. 提取 NaiveDate（去掉时间部分）
            let date: NaiveDate = dt.date_naive();
            let qty = item["qty"].as_f64().unwrap();
            repository.upsert_weight(date, qty).await?;
            println!(
                "数据同步:\n{}",
                serde_json::to_string_pretty(&item).unwrap()
            );
        }
    }
    Ok(())
}
