use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub enum Role {
    User,
    Assistant,
}

impl Role {
    const USER_STR: &'static str = "user";
    const ASSISTANT_STR: &'static str = "assistant";

    pub fn to_string(&self) -> String {
        match self {
            Role::User => Self::USER_STR.to_string(),
            Role::Assistant => Self::ASSISTANT_STR.to_string(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct ChatRequest<'a> {
    pub model: &'a str,
    pub messages: &'a [Message],
    pub stream: bool,
}

#[derive(Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize)]
pub struct Choice {
    pub message: ResponseMessage,
}

#[derive(Deserialize)]
pub struct ResponseMessage {
    pub content: String,
}
