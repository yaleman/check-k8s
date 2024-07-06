//! CLI parsing things

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Default)]
pub struct CliOpt {
    #[clap(short, long)]
    pub debug: bool,

    #[clap(short, long)]
    /// Specify an optional kubeconfig file (or use KUBECONFIG env var)
    pub kubeconfig: Option<PathBuf>,

    #[clap(short, long)]
    /// Filter by namespace
    pub namespace: Option<String>,
}
