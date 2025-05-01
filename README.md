# AI-MCP Playground

This is an experimental project that demonstrates the capabilities of AI-assisted development using Cursor IDE. The entire codebase was developed through a collaborative process between a human developer and an AI assistant, with minimal direct human interaction with the code.

## Project Overview

This MCP (Model Context Protocol) server is an experimental playground for integrating various services and testing their capabilities. Currently, it includes integrations with:
- GitHub Actions
- Jira

The project serves as a proof of concept for AI-assisted development workflows and showcases how AI can help in creating well-structured, modular applications. The specific tools and features are not the main focus - they are just examples of what can be built using this approach.

## Development Process

This project was developed using an iterative, conversational approach with an AI assistant in Cursor IDE. The development process included:
- Initial project setup and structure
- Implementation of GitHub integration
- Addition of Jira functionality
- Security improvements
- Environment configuration

Each step was guided by natural language conversations, with the AI assistant providing code suggestions, explanations, and best practices. The human developer's role was primarily focused on:
- Providing high-level requirements and direction
- Reviewing and accepting AI-suggested changes
- Making minor adjustments when needed
- Intervening only for significant architectural decisions or to prevent solution loops

## Current State

As an experimental project, the current implementation includes some example integrations, but these are subject to change as the project evolves. The focus is on the development process and architecture rather than specific features.

The modular design allows for easy addition, removal, or modification of service integrations, making it a flexible platform for testing different ideas and approaches.

## Features

- Modular service integration architecture
- Secure handling of sensitive information
- Environment-based configuration
- Extensible design for easy addition of new services

## Setup

1. Clone the repository
2. Copy `.env.example` to `.env`:
   ```bash
   cp .env.example .env
   ```
3. Fill in your environment variables in `.env`:
   - `GITHUB_TOKEN`: Your GitHub Personal Access Token
   - `GITHUB_ORG`: Your GitHub organization name
   - `JIRA_TOKEN`: Your Jira API token
   - `JIRA_BASE_URL`: Your Jira instance URL

4. Run the server:
   ```bash
   cargo run
   ```

## Project Structure

```
src/
├── main.rs          # Application entry point
├── service.rs       # Main service implementation
├── github/          # GitHub integration
│   ├── client.rs    # GitHub API client
│   └── service.rs   # GitHub service layer
└── jira/           # Jira integration
    ├── client.rs    # Jira API client
    └── service.rs   # Jira service layer
```

## Security Notes

- The project uses environment variables for sensitive configuration
- `.env` file is gitignored to prevent accidental commits of secrets
- Sensitive information is not exposed in tool responses

## Purpose

This project serves as an experiment in AI-assisted development, exploring:
- How AI can help in creating well-structured applications
- The effectiveness of conversational programming
- Best practices for AI-human collaboration in software development
- The potential of AI in modern development workflows

## Disclaimer

This is an experimental project created for testing and learning purposes. It demonstrates the capabilities of AI-assisted development but should not be used in production without proper review and testing.

## License

This project is open source and available under the MIT License. 