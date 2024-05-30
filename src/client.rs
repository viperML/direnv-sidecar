use std::collections::HashMap;
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
use serde::Serialize;
use tokio::io::unix::AsyncFd;
use tracing::{debug, trace};
use zbus::proxy;
use zbus::zvariant::DynamicType;
use zbus::zvariant::Type;
use zbus::zvariant::Value;
use zbus::Connection;

use crate::server::Data;

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

#[proxy(
    interface = "net.direnv.Sidecar",
    default_service = "net.direnv.Sidecar",
    default_path = "/net/direnv/Sidecar"
)]
trait Sidecar {
    fn register(&self, data: crate::server::Data) -> zbus::Result<crate::server::Response>;
}

/// https://man7.org/linux/man-pages/man2/pidfd_open.2.html
impl StartArgs {
    pub async fn run(self) -> eyre::Result<()> {
        let parent = direnv_parent()?;
        debug!(?parent);

        let connection = Connection::session().await?;

        let proxy = SidecarProxy::new(&connection).await?;
        proxy
            .register(Data {
                pid: parent.pid,
                direnv_diff: String::from(""),
            })
            .await?;

        Ok(())
    }
}
