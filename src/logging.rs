//! Where logging's configured

use tracing::*;
use tracing_subscriber::{fmt::format, EnvFilter};

use crate::cli::CliOpt;

fn output_formatter() -> tracing_subscriber::fmt::format::Format<format::Compact, ()> {
    // Configure a custom event formatter

    tracing_subscriber::fmt::format()
        .compact() // use an abbreviated format for logging spans
        .without_time()
        .with_level(false) // don't include levels in formatted output
        .with_target(false) // don't include targets
}

/// Configure logging based on the CLI options
pub fn configure_logging(opts: &CliOpt) -> anyhow::Result<()> {
    let log_level = if opts.debug { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_max_level(Level::ERROR)
        .event_format(output_formatter())
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive(log_level.parse()?)
                .add_directive("http=error".parse()?),
        )
        .init();

    Ok(())
}
