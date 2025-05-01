use anyhow::{Result, anyhow};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub key: String,
    pub fields: IssueFields,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueFields {
    pub summary: String,
    pub description: Option<String>,
    pub status: IssueStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct IssueStatus {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Transition {
    pub id: String,
    pub name: String,
    pub to: IssueStatus,
}

#[derive(Debug, Serialize)]
struct TransitionRequest {
    transition: TransitionId,
}

#[derive(Debug, Serialize)]
struct TransitionId {
    id: String,
}

#[derive(Clone)]
pub struct JiraClient {
    client: reqwest::Client,
    base_url: String,
}

impl JiraClient {
    pub fn new(token: String, base_url: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { client, base_url }
    }

    pub async fn get_issue(&self, issue_id: &str) -> Result<Issue> {
        let url = format!(
            "{}/rest/api/2/issue/{}",
            self.base_url.trim_end_matches('/'),
            issue_id
        );
        
        let response = self.client
            .get(&url)
            .send()
            .await?;

        let status = response.status();
        debug!("Jira API response status: {}", status);

        if !status.is_success() {
            let error_body = response.text().await?;
            anyhow::bail!("Jira API error: {} - {}", status, error_body);
        }

        let issue = response.json().await?;
        Ok(issue)
    }

    pub async fn get_transitions(&self, issue_id: &str) -> Result<Vec<Transition>> {
        let url = format!(
            "{}/rest/api/2/issue/{}/transitions",
            self.base_url.trim_end_matches('/'),
            issue_id
        );
        
        let response = self.client
            .get(&url)
            .send()
            .await?;

        let status = response.status();
        debug!("Jira API transitions response status: {}", status);

        if !status.is_success() {
            let error_body = response.text().await?;
            anyhow::bail!("Jira API error: {} - {}", status, error_body);
        }

        let transitions: serde_json::Value = response.json().await?;
        let transitions = serde_json::from_value::<Vec<Transition>>(transitions["transitions"].clone())?;
        Ok(transitions)
    }

    pub async fn do_transition(&self, issue_id: &str, transition_id: &str) -> Result<()> {
        let url = format!(
            "{}/rest/api/2/issue/{}/transitions",
            self.base_url.trim_end_matches('/'),
            issue_id
        );

        let request = TransitionRequest {
            transition: TransitionId {
                id: transition_id.to_string(),
            },
        };
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        debug!("Jira API transition response status: {}", status);

        if !status.is_success() {
            let error_body = response.text().await?;
            anyhow::bail!("Jira API error: {} - {}", status, error_body);
        }

        Ok(())
    }

    pub async fn advance_to_status(&self, issue_id: &str, target_status: &str) -> Result<()> {
        let mut issue = self.get_issue(issue_id).await?;
        
        while issue.fields.status.name != target_status {
            let transitions = self.get_transitions(issue_id).await?;
            
            // Try to find a direct transition to the target status
            if let Some(direct_transition) = transitions.iter().find(|t| t.to.name == target_status) {
                self.do_transition(issue_id, &direct_transition.id).await?;
                break;
            }

            // If no direct transition, find the first available transition that gets us closer
            let next_transition = transitions.first()
                .ok_or_else(|| anyhow!("No transitions available from status '{}'", issue.fields.status.name))?;

            self.do_transition(issue_id, &next_transition.id).await?;
            
            // Update issue status for next iteration
            issue = self.get_issue(issue_id).await?;
        }

        Ok(())
    }
} 