use tracing::instrument;
use zbus::{conn, interface, Connection, ObjectServer};

#[derive(Debug)]
struct Server;

#[interface(name = "net.direnv.Sidecar")]
impl Server {
    #[instrument(ret, level="debug")]
    async fn say_hello(&self, name: &str) -> String {
        format!("Hello {}!", name)
    }

    #[zbus(property)]
    fn myconst(&self) -> String {
        String::from("hello")
    }
}

pub async fn run() -> eyre::Result<()> {
    let _connection = conn::Builder::session()?
        .name("net.direnv.Sidecar")?
        .serve_at("/net/direnv/Sidecar", Server)?
        .build()
        .await?;

    loop {
        std::future::pending::<()>().await;
    }

    Ok(())
}
