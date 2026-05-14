use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "body-recorder")]
#[command(about = "Record daily body weight in kilograms")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Save Supabase connection settings to the local config file.
    Init {
        #[arg(long)]
        url: String,
        #[arg(long)]
        key: String,
    },
    /// Add or replace a weight record for one day.
    Add {
        weight_kg: f64,
        #[arg(long)]
        date: Option<String>,
    },
    /// List recent weight records.
    List {
        #[arg(long, default_value_t = 30)]
        limit: u32,
    },
    /// Update the weight for an existing date.
    Update { date: String, weight_kg: f64 },
    /// Delete the weight record for a date.
    Delete { date: String },
    /// Compare recent average weight with earlier periods.
    Compare {
        /// Reference date for the comparison. Defaults to today.
        #[arg(long)]
        date: Option<String>,
    },
    /// Print the Supabase SQL schema for this tool.
    Schema {
        #[arg(long, value_enum, default_value_t = AccessModel::ServiceRole)]
        access: AccessModel,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum AccessModel {
    /// Restrict REST table access to the service_role key.
    ServiceRole,
    /// Allow the anon key to read and write all weight records.
    Anon,
}
