use std::net::{Ipv4Addr, SocketAddr};

use adapter::database::connect_database_with;
use anyhow::{Error, Result};
use api::route::health::build_health_check_routers;

use axum::Router;
use registry::AppRegistry;
use shared::config::AppConfig;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    bootsrap().await
}

async fn bootsrap() -> Result<()> {
    let app_config = AppConfig::new()?;
    let pool = connect_database_with(&app_config.database);

    let registry = AppRegistry::new(pool);

    let app = Router::new()
        .merge(build_health_check_routers())
        .with_state(registry);

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listenr = TcpListener::bind(&addr).await?;
    println!("Listening on {}", addr);
    axum::serve(listenr, app).await.map_err(Error::from)
}

#[tokio::test]
async fn health_check_works() {}

#[sqlx::test]
async fn health_check_db_works() {}
