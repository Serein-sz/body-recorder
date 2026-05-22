use crate::error::{AppError, AppResult};
use chrono::{Local, NaiveDate};

pub fn validate_weight(weight_kg: f64) -> AppResult<f64> {
    if weight_kg.is_finite() && weight_kg > 0.0 && weight_kg < 1000.0 {
        Ok(weight_kg)
    } else {
        Err(AppError::InvalidWeight(weight_kg))
    }
}

pub fn parse_or_today(date: Option<String>) -> AppResult<NaiveDate> {
    match date {
        Some(value) => parse_date(&value),
        None => Ok(Local::now().date_naive()),
    }
}

pub fn parse_date(value: &str) -> AppResult<NaiveDate> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d").map_err(|_| AppError::InvalidDate(value.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_weight_range() {
        assert!(validate_weight(72.4).is_ok());
        assert!(validate_weight(0.0).is_err());
        assert!(validate_weight(-1.0).is_err());
        assert!(validate_weight(1000.0).is_err());
        assert!(validate_weight(f64::NAN).is_err());
    }

    #[test]
    fn parses_iso_date_only() {
        assert_eq!(
            parse_date("2026-05-14").unwrap(),
            NaiveDate::from_ymd_opt(2026, 5, 14).unwrap()
        );
        assert!(parse_date("2026/05/14").is_err());
    }
}
