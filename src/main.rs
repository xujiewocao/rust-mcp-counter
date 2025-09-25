use std::sync::Arc;

use rmcp::{
    handler::server::tool::ToolRouter, model::{
        CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
    }, tool, tool_handler, tool_router, transport::stdio, ServerHandler, ServiceExt
};
use tokio::{
    sync::Mutex,
};

// Run the server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and run the server with STDIO transport
    let service = HelloWorld::new().serve(stdio()).await.inspect_err(|e| {
        println!("Error starting server: {}", e);
    })?;
    service.waiting().await?;

    Ok(())
}

#[derive(Debug, Clone)]
pub struct HelloWorld {
    counter: Arc<Mutex<i32>>,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl HelloWorld {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0)),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Increment the counter")]
    async fn increment(&self) -> Result<CallToolResult, rmcp::ErrorData> {
        let mut counter = self.counter.lock().await;
        *counter += 1;
        Ok(CallToolResult::success(vec![Content::text(
            counter.to_string(),
        )]))
    }

    #[tool(description = "Decrement the counter")]
    async fn decrement(&self) -> Result<CallToolResult, rmcp::ErrorData> {
        let mut counter = self.counter.lock().await;
        *counter -= 1;
        Ok(CallToolResult::success(vec![Content::text(
            counter.to_string(),
        )]))
    }

    #[tool(description = "Get the current value of the counter")]
    async fn get_value(&self) -> Result<CallToolResult, rmcp::ErrorData> {
        let counter = self.counter.lock().await;
        Ok(CallToolResult::success(vec![Content::text(
            counter.to_string(),
        )]))
    }

    #[tool(description = "Reset the counter to zero")]
    async fn reset(&self) -> Result<CallToolResult, rmcp::ErrorData> {
        let mut counter = self.counter.lock().await;
        *counter = 0;
        Ok(CallToolResult::success(vec![Content::text(
            counter.to_string(),
        )]))
    }

    #[tool(description = "Echo the input")]
    async fn echo(&self) -> Result<CallToolResult, rmcp::ErrorData> {
        Ok(CallToolResult::success(vec![Content::text("你好， 我是mcp-server-rust")]))
    }
}

#[tool_handler]
impl ServerHandler for HelloWorld {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::LATEST,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("this is counter mcp server in rust".to_string()),
        }
    }
}