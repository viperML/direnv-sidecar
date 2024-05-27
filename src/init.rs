use clap::Parser;
use tracing::instrument;

use crate::Cli;

#[instrument(ret, level = "debug")]
pub fn init() -> Cli {
    {
        color_eyre::install().unwrap();
        use tracing_error::ErrorLayer;
        use tracing_subscriber::{fmt, prelude::*, EnvFilter};
        tracing_subscriber::registry()
            .with(fmt::layer().without_time().with_line_number(true))
            .with(EnvFilter::from_default_env())
            .with(ErrorLayer::default())
            .init();
    }

    Cli::parse()
}
