use crate::domain::models::Config;
use crate::error::{AppError, AppResult};
use reqwest::Url;
use std::fs;
use std::path::PathBuf;

const APP_DIR: &str = "body-recorder";
const CONFIG_FILE: &str = "config.json";

pub fn init_config(url: String, key: String) -> AppResult<PathBuf> {
    let url = normalize_supabase_url(&url)?;
    let config = Config {
        supabase_url: url,
        service_role_key: key,
    };
    let path = config_path()?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(&config)?;
    fs::write(&path, format!("{json}\n"))?;
    Ok(path)
}

pub fn read_config() -> AppResult<Config> {
    let path = config_path()?;
    if !path.exists() {
        return Err(AppError::MissingConfig(path));
    }

    let text = fs::read_to_string(path)?;
    let mut config: Config = serde_json::from_str(&text)?;
    config.supabase_url = normalize_supabase_url(&config.supabase_url)?;
    Ok(config)
}

fn config_path() -> AppResult<PathBuf> {
    let dir = dirs::config_dir().ok_or(AppError::ConfigDirUnavailable)?;
    Ok(dir.join(APP_DIR).join(CONFIG_FILE))
}

pub fn normalize_supabase_url(raw: &str) -> AppResult<String> {
    let trimmed = raw.trim().trim_end_matches('/');
    let parsed = Url::parse(trimmed)?;

    if parsed.scheme() != "https" && parsed.scheme() != "http" {
        return Err(AppError::Message(
            "Supabase URL must start with https:// or http://".to_string(),
        ));
    }

    let project_url = trimmed.trim_end_matches("/rest/v1").trim_end_matches('/');
    Ok(project_url.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_supabase_url() {
        assert_eq!(
            normalize_supabase_url("https://example.supabase.co/").unwrap(),
            "https://example.supabase.co"
        );
    }

    #[test]
    fn accepts_rest_endpoint_url() {
        assert_eq!(
            normalize_supabase_url("https://example.supabase.co/rest/v1").unwrap(),
            "https://example.supabase.co"
        );
    }
}
