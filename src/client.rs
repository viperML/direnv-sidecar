use std::ffi::OsString;

use clap::Args;
use clap::ValueHint;

#[derive(Debug, Args)]
pub struct StartArgs {
    #[arg(trailing_var_arg = true, num_args=1.., value_hint=ValueHint::CommandWithArguments)]
    command: Vec<OsString>,
}

impl StartArgs {
    pub fn run(self) -> eyre::Result<()> {
        Ok(())
    }
}
