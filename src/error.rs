use reqwest::StatusCode;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    ConfigDirUnavailable,
    MissingConfig(PathBuf),
    InvalidWeight(f64),
    InvalidDate(String),
    Http { status: StatusCode, body: String },
    Message(String),
    Io(std::io::Error),
    Json(serde_json::Error),
    Reqwest(reqwest::Error),
    Header(reqwest::header::InvalidHeaderValue),
    Url(url::ParseError),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConfigDirUnavailable => write!(f, "could not determine user config directory"),
            Self::MissingConfig(path) => write!(
                f,
                "missing config file: {}. Run `body-recorder init --url <SUPABASE_URL> --key <SERVICE_ROLE_KEY>` first",
                path.display()
            ),
            Self::InvalidWeight(value) => {
                write!(f, "invalid weight: {value}. Expected 0 < weight_kg < 1000")
            }
            Self::InvalidDate(value) => {
                write!(f, "invalid date: {value}. Expected YYYY-MM-DD")
            }
            Self::Http { status, body } => write!(f, "Supabase request failed ({status}): {body}"),
            Self::Message(message) => write!(f, "{message}"),
            Self::Io(error) => Display::fmt(error, f),
            Self::Json(error) => Display::fmt(error, f),
            Self::Reqwest(error) => Display::fmt(error, f),
            Self::Header(error) => Display::fmt(error, f),
            Self::Url(error) => Display::fmt(error, f),
        }
    }
}

impl Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for AppError {
    fn from(value: reqwest::header::InvalidHeaderValue) -> Self {
        Self::Header(value)
    }
}

impl From<url::ParseError> for AppError {
    fn from(value: url::ParseError) -> Self {
        Self::Url(value)
    }
}
