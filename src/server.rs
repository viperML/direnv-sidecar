use std::{
    collections::HashMap,
    os::fd::{FromRawFd, OwnedFd},
    sync::{Arc, Mutex},
};

use libc::{pid_t, syscall, SYS_pidfd_open};
use nix::errno::Errno;
use serde::{Deserialize, Serialize};
use tokio::io::unix::AsyncFd;
use tracing::{debug, instrument};
use zbus::{
    conn, interface, proxy,
    zvariant::{DeserializeDict, SerializeDict, Type},
    Connection, ObjectServer,
};

#[derive(Debug, Default)]
struct Server {
    services: Arc<Mutex<HashMap<pid_t, ClientInfo>>>,
}

#[derive(Debug)]
struct ClientInfo {
    direnv_diff: String,
}

unsafe fn pidfd_open(process: pid_t) -> nix::Result<OwnedFd> {
    let res = unsafe { syscall(SYS_pidfd_open, process, 0) };
    let raw = Errno::result(res)?;

    let fd = unsafe { OwnedFd::from_raw_fd(raw as _) };
    Ok(fd)
}

/// Waits for any PID, even if it's not a child
async fn wait(pid: pid_t) -> eyre::Result<()> {
    let fd = unsafe { pidfd_open(pid) }?;
    let afd = AsyncFd::new(fd)?;
    let _ = afd.readable().await?;

    Ok(())
}

#[derive(Debug, DeserializeDict, SerializeDict, Type)]
#[zvariant(signature = "a{sv}")]
pub struct Data {
    pub pid: pid_t,
    pub direnv_diff: String,
}

#[derive(Debug, SerializeDict, DeserializeDict, Type)]
#[zvariant(signature = "a{sv}")]
pub struct Response {}

#[interface(name = "net.direnv.Sidecar")]
impl Server {
    #[instrument(level = "debug", skip(self))]
    fn register(&self, data: Data) -> Response {
        debug!(?data);
        // let already_present = {
        //     let mut g = self.services.lock().unwrap();
        //     g.insert(pid, ClientInfo { direnv_diff }).is_some()
        // };

        // // self.services.insert(pid, ());
        // debug!(?self);

        // if !already_present {
        //     let services = self.services.clone();
        //     tokio::spawn(async move {
        //         wait(pid).await.unwrap();
        //         {
        //             let mut g = services.lock().unwrap();
        //             g.remove(&pid);
        //         }
        //         debug!(?pid, "finished");
        //     });
        // }
        Response{}
    }
}

pub async fn run() -> eyre::Result<()> {
    let _connection = conn::Builder::session()?
        .name("net.direnv.Sidecar")?
        .serve_at("/net/direnv/Sidecar", Server::default())?
        .build()
        .await?;

    loop {
        std::future::pending::<()>().await;
    }

    Ok(())
}
