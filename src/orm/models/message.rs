use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::core::prelude::Message;
use crate::orm::schema::message;

#[derive(Identifiable, PartialEq, Queryable, Debug)]
#[diesel(table_name = message)]
pub struct OrmMessage {
    pub id: String,
    pub text: String,
    pub answer_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Into<Message> for OrmMessage {
    fn into(self) -> Message {
        let answer_id = match self.answer_id {
            None => None,
            Some(value) => Some(value.clone()),
        };

        let mut message = Message::new(self.id.clone(), self.text.clone(), self.created_at);

        if answer_id.is_some() {
            message.set_answer_id(answer_id.clone().unwrap());
        }

        message
    }
}
