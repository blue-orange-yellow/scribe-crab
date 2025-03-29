use rmcp::{Error as McpError, ServerHandler, const_string, model::*, tool};
use std::fs;

#[derive(Debug, Clone)]
pub struct DocGenerator;

#[tool(tool_box)]
impl DocGenerator {
    pub fn new() -> Self {
        Self {}
    }

    #[tool(description = "Generate documentation for a function")]
    fn generate_doc(
        &self,
        #[tool(param)]
        #[schemars(description = "The Rust function code to document")]
        code: String,
    ) -> Result<CallToolResult, McpError> {
        // 環境変数からフォーマットファイルのパスを取得する
        let format_path = std::env::var("FORMAT_PATH").map_err(|e| {
            let error_message = format!("Failed to read format file at {}", e);
            McpError::internal_error(error_message, Some(e.to_string().into()))
        })?;

        // ファイルが存在するか確認
        if !std::path::Path::new(&format_path).exists() {
            return Err(McpError::internal_error(
                format!("Format file not found at {}", format_path),
                None,
            ));
        }

        let format = fs::read_to_string(&format_path).map_err(|e| {
            let error_message = format!("Failed to read format file at {}", format_path);
            McpError::internal_error(error_message, Some(e.to_string().into()))
        })?;

        let prompt_text = format!(
            "Use the following documentation format:\n\n{}\n\nNow write a documentation comment for this function:\n\n{}",
            format, code
        );

        // このツールの実際の実装では、モデルにプロンプトを送信してドキュメントを生成する
        // ここでは簡易的な実装として、プロンプトテキストを返します
        Ok(CallToolResult::success(vec![Content::text(format!(
            "Generated documentation for function:\n{}",
            prompt_text
        ))]))
    }
}

const_string!(GenerateDoc = "generate_doc");
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
