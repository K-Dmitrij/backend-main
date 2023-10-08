use serde::Serialize;

use crate::core::prelude::Message;

#[derive(Clone, Debug, Serialize)]
pub struct FindMessageDto {
    pub id: String,
    pub text: String,
    pub answer_id: Option<String>,
    pub created_at: String,
}

impl From<&Message> for FindMessageDto {
    fn from(message: &Message) -> Self {
        FindMessageDto {
            id: message.get_id(),
            text: message.get_text(),
            answer_id: message.get_anwer_id(),
            created_at: message
                .get_created_at()
                .format("%Y-%m-%dT%H:%M:%S.%fZ")
                .to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct MessageErrorDto {
    error: bool,
    message: String,
}

impl MessageErrorDto {
    pub fn new(message: &str) -> Self {
        MessageErrorDto {
            error: true,
            message: message.into(),
        }
    }
}
