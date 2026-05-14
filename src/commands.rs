use crate::cli::{Cli, Commands};
use crate::config::init_config;
use crate::error::AppResult;
use crate::schema::schema_sql;
use crate::stats::{ComparisonPoint, PeriodAverage, compare_weights, comparison_range};
use crate::supabase::SupabaseClient;
use crate::validation::{parse_date, parse_or_today, validate_weight};
use ansi_term::ANSIString;
use ansi_term::Colour::{Green, Red, Yellow};
use ansi_term::Style;
use clap::Parser;

pub async fn run() -> AppResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { url, key } => {
            let path = init_config(url, key)?;
            println!("saved config to {}", path.display());
            Ok(())
        }
        Commands::Schema { access } => {
            println!("{}", schema_sql(access));
            Ok(())
        }
        Commands::Add { weight_kg, date } => add_weight(weight_kg, date).await,
        Commands::List { limit } => list_weights(limit).await,
        Commands::Update { date, weight_kg } => update_weight(date, weight_kg).await,
        Commands::Delete { date } => delete_weight(date).await,
        Commands::Compare { date } => compare(date).await,
    }
}

async fn add_weight(weight_kg: f64, date: Option<String>) -> AppResult<()> {
    let client = SupabaseClient::from_config_file()?;
    let date = parse_or_today(date)?;
    let record = client
        .upsert_weight(date, validate_weight(weight_kg)?)
        .await?;

    println!("saved {} {:.2} kg", record.record_date, record.weight_kg);
    Ok(())
}

async fn list_weights(limit: u32) -> AppResult<()> {
    let client = SupabaseClient::from_config_file()?;
    let records = client.list_weights(limit).await?;

    if records.is_empty() {
        println!("no weight records found");
    } else {
        for record in records {
            println!("{} {:.2} kg", record.record_date, record.weight_kg);
        }
    }

    Ok(())
}

async fn update_weight(date: String, weight_kg: f64) -> AppResult<()> {
    let client = SupabaseClient::from_config_file()?;
    let date = parse_date(&date)?;
    let records = client
        .update_weight(date, validate_weight(weight_kg)?)
        .await?;

    match records.first() {
        Some(record) => println!("updated {} {:.2} kg", record.record_date, record.weight_kg),
        None => println!("no record found for {date}"),
    }

    Ok(())
}

async fn delete_weight(date: String) -> AppResult<()> {
    let client = SupabaseClient::from_config_file()?;
    let date = parse_date(&date)?;
    let records = client.delete_weight(date).await?;

    if records.is_empty() {
        println!("no record found for {date}");
    } else {
        println!("deleted {date}");
    }

    Ok(())
}

async fn compare(date: Option<String>) -> AppResult<()> {
    let reference_date = parse_or_today(date)?;
    let (start, end) = comparison_range(reference_date);
    let client = SupabaseClient::from_config_file()?;
    let records = client.list_weights_between(start, end).await?;
    let comparison = compare_weights(&records, reference_date);

    print_comparison(
        reference_date,
        records.len(),
        &comparison.recent_average,
        &comparison.points,
    );

    Ok(())
}

fn print_comparison(
    reference_date: chrono::NaiveDate,
    total_records: usize,
    recent: &PeriodAverage,
    points: &[ComparisonPoint],
) {
    let title = Style::new().bold().paint("Weight comparison");
    println!("{title}");
    println!("reference date: {reference_date}   records loaded: {total_records}");
    println!();

    println!("{}", Style::new().bold().paint("Baseline"));
    println!("{:<18} {:>10} {:>8}  range", "period", "average", "records");
    println!("{}", "-".repeat(62));
    println!(
        "{:<18} {:>10} {:>8}  {} to {}",
        recent.label,
        format_average(recent.average_kg),
        recent.sample_count,
        recent.start,
        recent.end
    );
    println!();

    println!("{}", Style::new().bold().paint("Compared with baseline"));
    println!(
        "{:<14} {:<10} {:>10} {:>10} {:>8}  {:<10} range",
        "period", "target", "average", "delta", "records", "status"
    );
    println!("{}", "-".repeat(98));

    for point in points {
        let delta = paint_delta(point.delta_from_recent_kg);
        let status = paint_status(point.delta_from_recent_kg);
        println!(
            "{:<14} {:<10} {:>10} {} {:>8}  {} {} to {}",
            point.label,
            point.target_date,
            format_average(point.average_kg),
            delta,
            point.sample_count,
            status,
            point.start,
            point.end
        );
    }
}

fn format_average(value: Option<f64>) -> String {
    value
        .map(|average| format!("{average:.2} kg"))
        .unwrap_or_else(|| "no data".to_string())
}

fn paint_delta(value: Option<f64>) -> ANSIString<'static> {
    let text = format!("{:>10}", format_delta(value));

    match value {
        Some(delta) if delta < -0.05 => Green.paint(text),
        Some(delta) if delta > 0.05 => Red.paint(text),
        Some(_) => Yellow.paint(text),
        None => Style::new().paint(text),
    }
}

fn format_delta(value: Option<f64>) -> String {
    value
        .map(|delta| format!("{delta:+.2} kg"))
        .unwrap_or_else(|| "n/a".to_string())
}

fn paint_status(value: Option<f64>) -> ANSIString<'static> {
    let text = format!("{:<10}", format_status(value));

    match value {
        Some(delta) if delta < -0.05 => Green.paint(text),
        Some(delta) if delta > 0.05 => Red.paint(text),
        Some(_) => Yellow.paint(text),
        None => Style::new().paint(text),
    }
}

fn format_status(value: Option<f64>) -> &'static str {
    match value {
        Some(delta) if delta < -0.05 => "lower",
        Some(delta) if delta > 0.05 => "higher",
        Some(_) => "steady",
        None => "missing",
    }
}
