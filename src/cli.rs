use clap::Parser;

#[derive(Parser, Default)]
pub struct CliOpt {
    #[clap(short, long)]
    pub debug: bool,

    #[clap(short, long)]
    pub namespace: Option<String>,
}
