//! CLI parsing things

use clap::Parser;

#[derive(Parser, Default)]
pub struct CliOpt {
    #[clap(short, long)]
    pub debug: bool,

    #[clap(short, long)]
    /// Filter by namespace
    pub namespace: Option<String>,
}
