use crate::domain::models::{Config, WeightRecord};
use crate::error::{AppError, AppResult};
use crate::storage::config::read_config;
use crate::storage::repository::WeightRepository;
use async_trait::async_trait;
use chrono::NaiveDate;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use reqwest::{Client, Method, Url};
use serde::{Deserialize, Serialize};

const TABLE: &str = "weight_records";

#[derive(Debug, Serialize)]
struct WeightPayload {
    record_date: NaiveDate,
    weight_kg: f64,
}

#[derive(Debug, Serialize)]
struct WeightUpdatePayload {
    weight_kg: f64,
}

pub struct SupabaseClient {
    http: Client,
    base_url: String,
    headers: HeaderMap,
}

impl SupabaseClient {
    pub fn from_config_file() -> AppResult<Self> {
        Self::new(read_config()?)
    }

    pub fn new(config: Config) -> AppResult<Self> {
        let mut headers = HeaderMap::new();
        let key = HeaderValue::from_str(&config.service_role_key)?;
        let auth = HeaderValue::from_str(&format!("Bearer {}", config.service_role_key))?;

        headers.insert("apikey", key);
        headers.insert("Authorization", auth);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        Ok(Self {
            http: Client::new(),
            base_url: config.supabase_url,
            headers,
        })
    }

    pub async fn upsert_weight(
        &self,
        record_date: NaiveDate,
        weight_kg: f64,
    ) -> AppResult<WeightRecord> {
        let mut url = self.table_url()?;
        url.query_pairs_mut()
            .append_pair("on_conflict", "record_date");

        let payload = WeightPayload {
            record_date,
            weight_kg,
        };
        let records: Vec<WeightRecord> = self
            .send_json(
                Method::POST,
                url,
                Some("resolution=merge-duplicates,return=representation"),
                &payload,
            )
            .await?;

        records
            .into_iter()
            .next()
            .ok_or_else(|| AppError::Message("Supabase did not return the saved record".into()))
    }

    pub async fn list_weights(&self, limit: u32) -> AppResult<Vec<WeightRecord>> {
        let mut url = self.table_url()?;
        url.query_pairs_mut()
            .append_pair("select", "record_date,weight_kg,created_at,updated_at")
            .append_pair("order", "record_date.desc")
            .append_pair("limit", &limit.to_string());

        self.send(Method::GET, url, None).await
    }

    pub async fn list_weights_between(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> AppResult<Vec<WeightRecord>> {
        let mut url = self.table_url()?;
        url.query_pairs_mut()
            .append_pair("select", "record_date,weight_kg,created_at,updated_at")
            .append_pair("record_date", &format!("gte.{start}"))
            .append_pair("record_date", &format!("lte.{end}"))
            .append_pair("order", "record_date.asc");

        self.send(Method::GET, url, None).await
    }

    pub async fn update_weight(
        &self,
        record_date: NaiveDate,
        weight_kg: f64,
    ) -> AppResult<Vec<WeightRecord>> {
        let mut url = self.table_url()?;
        url.query_pairs_mut()
            .append_pair("record_date", &format!("eq.{record_date}"));

        let payload = WeightUpdatePayload { weight_kg };
        self.send_json(Method::PATCH, url, Some("return=representation"), &payload)
            .await
    }

    pub async fn delete_weight(&self, record_date: NaiveDate) -> AppResult<Vec<WeightRecord>> {
        let mut url = self.table_url()?;
        url.query_pairs_mut()
            .append_pair("record_date", &format!("eq.{record_date}"));

        self.send(Method::DELETE, url, Some("return=representation"))
            .await
    }

    pub fn table_url(&self) -> AppResult<Url> {
        Ok(Url::parse(&format!(
            "{}/rest/v1/{}",
            self.base_url.trim_end_matches('/'),
            TABLE
        ))?)
    }

    async fn send<T>(&self, method: Method, url: Url, prefer: Option<&str>) -> AppResult<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut request = self.http.request(method, url).headers(self.headers.clone());

        if let Some(value) = prefer {
            request = request.header("Prefer", value);
        }

        let response = request.send().await?;
        parse_response(response).await
    }

    async fn send_json<T, B>(
        &self,
        method: Method,
        url: Url,
        prefer: Option<&str>,
        body: &B,
    ) -> AppResult<T>
    where
        T: for<'de> Deserialize<'de>,
        B: Serialize + ?Sized,
    {
        let mut request = self
            .http
            .request(method, url)
            .headers(self.headers.clone())
            .json(body);

        if let Some(value) = prefer {
            request = request.header("Prefer", value);
        }

        let response = request.send().await?;
        parse_response(response).await
    }
}

#[async_trait]
impl WeightRepository for SupabaseClient {
    async fn upsert_weight(
        &self,
        record_date: NaiveDate,
        weight_kg: f64,
    ) -> AppResult<WeightRecord> {
        SupabaseClient::upsert_weight(self, record_date, weight_kg).await
    }

    async fn list_weights(&self, limit: u32) -> AppResult<Vec<WeightRecord>> {
        SupabaseClient::list_weights(self, limit).await
    }

    async fn list_weights_between(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> AppResult<Vec<WeightRecord>> {
        SupabaseClient::list_weights_between(self, start, end).await
    }

    async fn update_weight(
        &self,
        record_date: NaiveDate,
        weight_kg: f64,
    ) -> AppResult<Vec<WeightRecord>> {
        SupabaseClient::update_weight(self, record_date, weight_kg).await
    }

    async fn delete_weight(&self, record_date: NaiveDate) -> AppResult<Vec<WeightRecord>> {
        SupabaseClient::delete_weight(self, record_date).await
    }
}

async fn parse_response<T>(response: reqwest::Response) -> AppResult<T>
where
    T: for<'de> Deserialize<'de>,
{
    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        return Err(AppError::Http { status, body });
    }

    Ok(serde_json::from_str(&body)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_table_url() {
        let client = SupabaseClient::new(Config {
            supabase_url: "https://example.supabase.co".to_string(),
            service_role_key: "test-key".to_string(),
            profile: Default::default(),
        })
        .unwrap();

        assert_eq!(
            client.table_url().unwrap().as_str(),
            "https://example.supabase.co/rest/v1/weight_records"
        );
    }
}
