#[test]
fn get_id() {
    use crate::core::prelude::*;
    use chrono::Utc;
    use uuid::Uuid;

    let uuid = Uuid::new_v4().to_string();
    let message = Message::new(uuid.clone(), "New Message".into(), Utc::now());

    assert_eq!(uuid, message.get_id());
}

#[test]
fn get_text() {
    use crate::core::prelude::*;
    use chrono::Utc;
    use uuid::Uuid;

    let text = "New Message";
    let uuid = Uuid::new_v4().to_string();
    let message = Message::new(uuid.into(), text.into(), Utc::now());

    assert_eq!(text, message.get_text());
}

#[test]
fn get_answer_id() {
    use crate::core::prelude::*;
    use chrono::Utc;
    use uuid::Uuid;

    let text = "Text";
    let uuid = Uuid::new_v4().to_string();
    let uuid_v2 = Uuid::new_v4().to_string();
    let message = Message::new(uuid.clone(), text.into(), Utc::now());
    let mut message_v2 = Message::new(uuid_v2.into(), text.into(), Utc::now());

    message_v2.set_answer_id(uuid.clone());

    assert_eq!(None, message.get_anwer_id());
    assert_eq!(Some(uuid.clone()), message_v2.get_anwer_id());
}

#[test]
fn has_answer() {
    use crate::core::prelude::*;
    use chrono::Utc;
    use uuid::Uuid;

    let text = "Text";
    let uuid = Uuid::new_v4().to_string();
    let uuid_v2 = Uuid::new_v4().to_string();
    let message = Message::new(uuid.clone(), text.into(), Utc::now());
    let mut message_v2 = Message::new(uuid_v2, text.into(), Utc::now());

    message_v2.set_answer_id(uuid.into());

    assert_eq!(false, message.has_answer());
    assert_eq!(true, message_v2.has_answer());
}
