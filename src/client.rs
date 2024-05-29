use std::ffi::OsString;
use std::fs::File;
use std::io;
use std::os::fd::FromRawFd;
use std::os::fd::OwnedFd;

use clap::Args;
use clap::ValueHint;
use libc::pid_t;
use libc::syscall;
use libc::SYS_pidfd_getfd;
use libc::SYS_pidfd_open;
use nix::errno::Errno;
use nix::unistd::getpid;
use nix::unistd::getppid;
use procfs::process::Process;
use procfs::ProcResult;
use tokio::io::unix::AsyncFd;
use tracing::{debug, trace};
use zbus::Connection;

#[derive(Debug, Args)]
pub struct StartArgs {
    #[arg(trailing_var_arg = true, num_args=1.., value_hint=ValueHint::CommandWithArguments)]
    command: Vec<OsString>,
}

fn parent(proc: &Process) -> ProcResult<Process> {
    let status = proc.status()?;
    Process::new(status.ppid)
}

fn direnv_parent() -> ProcResult<Process> {
    let mut proc = Process::myself()?;
    loop {
        let status = proc.status()?;
        trace!(?status);
        proc = parent(&proc)?;

        if status.name == "direnv" {
            break;
        }
    }
    Ok(proc)
}

/// https://man7.org/linux/man-pages/man2/pidfd_open.2.html
impl StartArgs {
    pub async fn run(self) -> eyre::Result<()> {
        let parent = direnv_parent()?;
        debug!(?parent);

        // let pid: pid_t = 37634;

        // debug!("waiting pid!");
        // wait(pid).await?;

        let connection = Connection::session().await?;
        let resp = connection
            .call_method(
                Some("net.direnv.Sidecar"),
                "/net/direnv/Sidecar",
                Some("net.direnv.Sidecar"),
                "Register",
                &(parent.pid),
            )
            .await?;
        debug!(?resp);


        Ok(())
    }
}
