use crate::cli::{Cli, Commands};
use crate::config::init_config;
use crate::error::AppResult;
use crate::output;
use crate::schema::schema_sql;
use crate::supabase::SupabaseClient;
use crate::tui;
use crate::use_cases;
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
        Commands::Add { weight_kg, date } => {
            let repository = SupabaseClient::from_config_file()?;
            let result = use_cases::add_weight(&repository, weight_kg, date).await?;
            print!("{}", output::render_add_weight(&result));
            Ok(())
        }
        Commands::List { limit } => {
            let repository = SupabaseClient::from_config_file()?;
            let result = use_cases::list_weights(&repository, limit).await?;
            print!("{}", output::render_list_weights(&result.records));
            Ok(())
        }
        Commands::Update { date, weight_kg } => {
            let repository = SupabaseClient::from_config_file()?;
            let result = use_cases::update_weight(&repository, date, weight_kg).await?;
            print!("{}", output::render_update_weight(&result));
            Ok(())
        }
        Commands::Delete { date } => {
            let repository = SupabaseClient::from_config_file()?;
            let result = use_cases::delete_weight(&repository, date).await?;
            print!("{}", output::render_delete_weight(&result));
            Ok(())
        }
        Commands::Compare { date } => {
            let repository = SupabaseClient::from_config_file()?;
            let result = use_cases::compare(&repository, date).await?;
            print!(
                "{}",
                output::render_comparison(
                    result.reference_date,
                    result.total_records,
                    &result.comparison.recent_average,
                    &result.comparison.points,
                )
            );
            Ok(())
        }
        Commands::Advice { goal, date } => {
            let repository = SupabaseClient::from_config_file()?;
            let result = use_cases::advice(&repository, goal, date).await?;
            print!("{}", output::render_advice(&result.advice));
            Ok(())
        }
        Commands::Target { weight_kg, date } => {
            let repository = SupabaseClient::from_config_file()?;
            let result = use_cases::target(&repository, weight_kg, date).await?;
            print!("{}", output::render_target_projection(&result.projection));
            Ok(())
        }
        Commands::Tui => {
            let repository = SupabaseClient::from_config_file()?;
            tui::run(&repository).await
        }
    }
}
