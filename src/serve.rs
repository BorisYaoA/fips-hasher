use axum::{Router, routing::get};
use std::net::SocketAddr;

pub async fn run_server(addr: String) -> anyhow::Result<()> {
    async fn root() -> &'static str { "fips-hasher API" }
    let app = Router::new().route("/", get(root));

    let addr: SocketAddr = addr.parse()?;
    axum::Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}