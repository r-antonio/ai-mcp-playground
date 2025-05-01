use anyhow::Result;
use rmcp::{
    Error as McpError,
    model::*,
    tool,
    ServerHandler,
};
use serde_json::json;

use crate::github::GitHubService;
use crate::jira::JiraService;

#[derive(Clone)]
pub struct Service {
    github: GitHubService,
    jira: JiraService,
}

#[tool(tool_box)]
impl Service {
    pub fn new(
        github_token: String,
        github_org: String,
        jira_token: String,
        jira_base_url: String,
    ) -> Self {
        Self {
            github: GitHubService::new(github_token, github_org),
            jira: JiraService::new(jira_token, jira_base_url),
        }
    }

    #[tool(description = "List all GitHub Actions runners in the organization")]
    async fn list_runners(&self) -> Result<CallToolResult, McpError> {
        let runners = self.github.list_runners().await
            .map_err(|e| McpError::internal_error("Failed to fetch runners", Some(json!({ "error": e.to_string() }))))?;
        Ok(CallToolResult::success(vec![Content::text(
            json!(runners).to_string()
        )]))
    }

    #[tool(description = "Get a Jira issue by its ID")]
    async fn get_issue(
        &self,
        #[tool(param)]
        #[schemars(description = "The ID of the issue to fetch")]
        issue_id: String,
    ) -> Result<CallToolResult, McpError> {
        let issue = self.jira.get_issue(&issue_id).await
            .map_err(|e| McpError::internal_error("Failed to fetch issue", Some(json!({ "error": e.to_string() }))))?;
        Ok(CallToolResult::success(vec![Content::text(
            json!({
                "id": issue.id,
                "key": issue.key
            }).to_string()
        )]))
    }

    #[tool(description = "Advance a Jira issue to a specific status")]
    async fn advance_issue_status(
        &self,
        #[tool(param)]
        #[schemars(description = "The ID of the issue to update")]
        issue_id: String,
        #[tool(param)]
        #[schemars(description = "The target status to advance to")]
        target_status: String,
    ) -> Result<CallToolResult, McpError> {
        self.jira.advance_status(&issue_id, &target_status).await
            .map_err(|e| McpError::internal_error("Failed to advance issue status", Some(json!({ "error": e.to_string() }))))?;
        
        Ok(CallToolResult::success(vec![Content::text(
            json!({
                "message": format!("Successfully advanced issue {} to status '{}'", issue_id, target_status)
            }).to_string()
        )]))
    }

    #[tool(description = "Add a version to a Jira issue")]
    async fn add_issue_version(
        &self,
        #[tool(param)]
        #[schemars(description = "The ID of the issue to update")]
        issue_id: String,
        #[tool(param)]
        #[schemars(description = "The ID of the version to add to the issue")]
        version_id: String,
    ) -> Result<CallToolResult, McpError> {
        self.jira.add_version(&issue_id, &version_id).await
            .map_err(|e| McpError::internal_error("Failed to add version to issue", Some(json!({ "error": e.to_string() }))))?;
        
        Ok(CallToolResult::success(vec![Content::text(
            json!({
                "message": format!("Successfully added version {} to issue {}", version_id, issue_id)
            }).to_string()
        )]))
    }
}

#[tool(tool_box)]
impl ServerHandler for Service {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides tools to interact with GitHub Actions runners and Jira issues.".to_string()),
        }
    }
}
