use std::sync::Arc;

use diesel::{insert_into, prelude::*};

use crate::orm::db::Db;
use crate::orm::models::message::*;
use crate::orm::schema::message::dsl::*;

#[derive(Clone)]
pub struct MessageTable {
    db: Arc<Db>,
}

impl MessageTable {
    pub fn new(db: Arc<Db>) -> MessageTable {
        MessageTable { db }
    }

    pub fn find_all(&self) -> Result<Vec<OrmMessage>, diesel::result::Error> {
        self.db.apply(|conn| message.load::<OrmMessage>(conn))
    }

    pub fn find(&self, message_id: String) -> Option<OrmMessage> {
        self.db
            .apply(|conn| match message.find(message_id.clone()).first(conn) {
                Ok(res) => Some(res),
                Err(_) => None,
            })
    }

    pub fn find_all_topics(&self) -> Result<Vec<OrmMessage>, diesel::result::Error> {
        self.db
            .apply(|conn| message.filter(answer_id.is_null()).load::<OrmMessage>(conn))
    }

    pub fn find_topic(&self, message_id: String) -> Option<OrmMessage> {
        match self.find(message_id) {
            Some(res) => {
                if res.answer_id.is_none() {
                    Some(res)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn insert(
        &self,
        message_text: String,
        message_anwer: Option<String>,
    ) -> Result<Vec<String>, diesel::result::Error> {
        self.db.apply(|conn| {
            insert_into(message)
                .values((
                    text.eq(message_text.clone()),
                    answer_id.eq(message_anwer.clone()),
                ))
                .returning(id)
                .get_results(conn)
        })
    }
}
