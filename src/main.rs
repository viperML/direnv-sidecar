mod init;

use clap::Parser;
use tracing::debug;


#[derive(Debug, Parser)]
struct Cli {}

#[tokio::main]
async fn main() {
    let cli = init::init();

    debug!("Hello");
}
