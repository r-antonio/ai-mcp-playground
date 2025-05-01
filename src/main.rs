use rmcp::transport::sse_server::SseServer;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    {self},
};
mod common;
mod github;
mod jira;
mod service;

use service::Service;

const BIND_ADDRESS: &str = "127.0.0.1:8000";

fn load_env() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file
    load_env()?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let server = SseServer::serve(BIND_ADDRESS.parse()?)
        .await?
        .with_service(|| {
            Service::new(
                std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set"),
                std::env::var("GITHUB_ORG").expect("GITHUB_ORG must be set"),
                std::env::var("JIRA_TOKEN").expect("JIRA_TOKEN must be set"),
                std::env::var("JIRA_BASE_URL").expect("JIRA_BASE_URL must be set"),
            )
        });

    tokio::signal::ctrl_c().await?;
    server.cancel();
    Ok(())
}