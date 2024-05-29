#![deny(unsafe_op_in_unsafe_fn)]

mod client;
mod init;
mod server;

use clap::Parser;
use tracing::debug;

#[derive(Debug, Parser)]
enum Cli {
    Server,
    Completions {
        #[arg(short, long)]
        shell: clap_complete::Shell,
    },
    Start(crate::client::StartArgs),
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cli = init::init();

    match cli {
        Cli::Completions { shell } => {
            let mut cmd = <Cli as clap::CommandFactory>::command();
            clap_complete::generate(shell, &mut cmd, "direnv-sidecar", &mut std::io::stdout());
            Ok(())
        }
        Cli::Server => crate::server::run().await,
        Cli::Start(args) => args.run().await,
    }
}
