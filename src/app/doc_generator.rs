use rmcp::{Error as McpError, ServerHandler, const_string, model::*, tool};

#[derive(Debug, Clone)]
pub struct DocGenerator;

#[tool(tool_box)]
impl DocGenerator {
    pub fn new() -> Self {
        Self {}
    }

    #[tool(description = "Repeat what you say")]
    fn echo(
        &self,
        #[tool(param)]
        #[schemars(description = "Repeat what you say")]
        saying: String,
    ) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text(saying)]))
    }
}

const_string!(Echo = "echo");
#[tool(tool_box)]
impl ServerHandler for DocGenerator {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides a counter tool that can increment and decrement values. The counter starts at 0 and can be modified using the 'increment' and 'decrement' tools. Use 'get_value' to check the current count.".to_string()),
        }
    }
}
