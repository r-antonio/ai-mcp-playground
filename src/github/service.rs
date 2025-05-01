use anyhow::Result;
use super::client::GitHubClient;

#[derive(Clone)]
pub struct GitHubService {
    client: GitHubClient,
}

impl GitHubService {
    pub fn new(token: String, org: String) -> Self {
        Self {
            client: GitHubClient::new(token, org),
        }
    }

    pub async fn list_runners(&self) -> Result<Vec<super::client::Runner>> {
        self.client.list_runners().await
    }
}