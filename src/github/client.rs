use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct Runner {
    pub id: i64,
    pub name: String,
    pub os: String,
    pub status: String,
    pub busy: bool,
}

#[derive(Clone)]
pub struct GitHubClient {
    client: reqwest::Client,
    org: String,
}

impl GitHubClient {
    pub fn new(token: String, org: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("mcp-github-service"),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { client, org }
    }

    pub async fn list_runners(&self) -> Result<Vec<Runner>> {
        let url = format!(
            "https://api.github.com/orgs/{}/actions/runners",
            self.org
        );
        
        let response = self.client
            .get(&url)
            .send()
            .await?;

        let status = response.status();
        debug!("GitHub API response status: {}", status);

        if !status.is_success() {
            let error_body = response.text().await?;
            anyhow::bail!("GitHub API error: {} - {}", status, error_body);
        }

        let parsed: serde_json::Value = response.json().await?;
        let runners = serde_json::from_value::<Vec<Runner>>(
            parsed["runners"].clone()
        )?;

        Ok(runners)
    }
} 