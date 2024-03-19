use clap::{Parser, Subcommand};
use std::fmt;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand, action = ArgRequiredElseHelp)]
    pub(crate) command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    Import { path: String },
    Start { name: String },
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Commands::Import { path } => write!(f, "Import {}", path),
            Commands::Start { name } => write!(f, "Start {}", name),
        }
    }
}
