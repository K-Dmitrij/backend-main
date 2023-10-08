use chrono::{DateTime, Utc};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Message {
    id: String,
    text: String,
    anwer_id: Option<String>,
    created_at: DateTime<Utc>,
}

impl Message {
    pub fn new(id: String, text: String, created_at: DateTime<Utc>) -> Message {
        Message {
            id,
            text,
            anwer_id: None,
            created_at,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn get_anwer_id(&self) -> Option<String> {
        if self.has_answer() {
            return Some(self.anwer_id.clone().unwrap());
        }

        None
    }

    pub fn get_created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn set_id(&mut self, id: String) -> &mut Self {
        self.id = id;

        self
    }

    pub fn set_text(&mut self, text: String) -> &mut Self {
        self.text = text;

        self
    }

    pub fn set_answer_id(&mut self, anwer_id: String) -> &mut Self {
        self.anwer_id = Some(anwer_id);

        self
    }

    pub fn has_answer(&self) -> bool {
        self.anwer_id.is_some()
    }
}
