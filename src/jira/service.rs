use anyhow::Result;
use super::client::{JiraClient, Issue};

#[derive(Clone)]
pub struct JiraService {
    client: JiraClient,
}

impl JiraService {
    pub fn new(token: String, base_url: String) -> Self {
        Self {
            client: JiraClient::new(token, base_url),
        }
    }

    pub async fn get_issue(&self, issue_id: &str) -> Result<Issue> {
        self.client.get_issue(issue_id).await
    }

    pub async fn advance_status(&self, issue_id: &str, target_status: &str) -> Result<()> {
        self.client.advance_to_status(issue_id, target_status).await
    }
} 