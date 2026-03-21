use crate::data_models::chat::{ChatRequest, ChatResponse, Message};
use crate::settings::api;

use reqwest::Client;

pub async fn send_message(
    client: &Client,
    api_key: &str,
    history: &[Message],
) -> Result<String, String> {
    let request_body = ChatRequest {
        model: "deepseek-chat",
        messages: history,
        stream: false,
    };

    let response = client
        .post(api::DEEPSEEK_URI)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("HTTP {status}: {body}"));
    }

    let parsed: ChatResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {e}"))?;

    parsed
        .choices
        .into_iter()
        .next()
        .map(|c| c.message.content)
        .ok_or_else(|| "Empty response from API".to_string())
}
