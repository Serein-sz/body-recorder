use crate::domain::goals as domain_goals;
use crate::domain::schema as domain_schema;
use crate::domain::stats::DEFAULT_TARGET_WEIGHT_KG;
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
    /// Analyze recent weight trend and suggest a conservative diet adjustment.
    Advice {
        /// Goal to interpret the trend against. Defaults to cut.
        goal: Option<AdviceGoal>,
        /// Reference date for the advice. Defaults to today.
        #[arg(long)]
        date: Option<String>,
    },
    /// Estimate progress toward a target weight from the recent 4-week trend.
    Target {
        /// Target body weight in kilograms. Defaults to 70.0.
        #[arg(long, default_value_t = DEFAULT_TARGET_WEIGHT_KG)]
        weight_kg: f64,
        /// Reference date for the estimate. Defaults to today.
        #[arg(long)]
        date: Option<String>,
    },
    /// Print the Supabase SQL schema for this tool.
    Schema {
        #[arg(long, value_enum, default_value_t = AccessModel::ServiceRole)]
        access: AccessModel,
    },
    /// Open the interactive terminal interface.
    Tui,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum AccessModel {
    /// Restrict REST table access to the service_role key.
    ServiceRole,
    /// Allow the anon key to read and write all weight records.
    Anon,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum AdviceGoal {
    /// Lose body weight.
    Cut,
    /// Keep body weight stable.
    Maintain,
    /// Gain body weight.
    Gain,
}

impl From<AccessModel> for domain_schema::AccessModel {
    fn from(value: AccessModel) -> Self {
        match value {
            AccessModel::ServiceRole => Self::ServiceRole,
            AccessModel::Anon => Self::Anon,
        }
    }
}

impl From<AdviceGoal> for domain_goals::AdviceGoal {
    fn from(value: AdviceGoal) -> Self {
        match value {
            AdviceGoal::Cut => Self::Cut,
            AdviceGoal::Maintain => Self::Maintain,
            AdviceGoal::Gain => Self::Gain,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advice_goal_defaults_to_cut_when_omitted() {
        let cli = Cli::try_parse_from(["br", "advice"]).unwrap();

        match cli.command {
            Commands::Advice { goal, .. } => {
                assert_eq!(goal.unwrap_or(AdviceGoal::Cut), AdviceGoal::Cut)
            }
            _ => panic!("expected advice command"),
        }
    }

    #[test]
    fn advice_goal_accepts_explicit_goal() {
        let cli = Cli::try_parse_from(["br", "advice", "maintain"]).unwrap();

        match cli.command {
            Commands::Advice { goal, .. } => assert_eq!(goal, Some(AdviceGoal::Maintain)),
            _ => panic!("expected advice command"),
        }
    }

    #[test]
    fn accepts_tui_command() {
        let cli = Cli::try_parse_from(["br", "tui"]).unwrap();

        assert!(matches!(cli.command, Commands::Tui));
    }

    #[test]
    fn target_weight_defaults_to_70kg() {
        let cli = Cli::try_parse_from(["br", "target"]).unwrap();

        match cli.command {
            Commands::Target { weight_kg, .. } => assert_eq!(weight_kg, 70.0),
            _ => panic!("expected target command"),
        }
    }
}
