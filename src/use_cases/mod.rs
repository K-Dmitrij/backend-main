pub mod dto;

use crate::{core::prelude::Message, orm::tables::message::MessageTable};
use dto::FindMessageDto;

use self::dto::MessageErrorDto;

pub fn get_all_message(table: &MessageTable) -> Result<Vec<FindMessageDto>, MessageErrorDto> {
    let result = table.find_all();

    if result.is_err() {
        return Err(MessageErrorDto::new("Failed to load records"));
    }

    Ok(result
        .unwrap()
        .into_iter()
        .map(|message| {
            let core_message = Into::<Message>::into(message);
            From::from(&core_message)
        })
        .collect())
}

pub fn get_message(table: &MessageTable, id: String) -> Option<FindMessageDto> {
    let message = table.find(id);

    if message.is_none() {
        return None;
    }

    let message: Message = message.unwrap().into();

    Some(FindMessageDto::from(&message))
}

pub fn post_topic(table: &MessageTable, text: String) -> Result<Vec<String>, MessageErrorDto> {
    let result = table.insert(text, None);

    if result.is_err() {
        return Err(MessageErrorDto::new("Cannot insert into table"));
    }

    Ok(result.unwrap())
}

pub fn post_message(
    table: &MessageTable,
    text: String,
    answer_id: String,
) -> Result<Vec<String>, MessageErrorDto> {
    let result = table.insert(text, Some(answer_id));

    if result.is_err() {
        return Err(MessageErrorDto::new("Cannot insert into table"));
    }

    Ok(result.unwrap())
}

pub fn get_all_topics(table: &MessageTable) -> Result<Vec<FindMessageDto>, MessageErrorDto> {
    let result = table.find_all_topics();

    if result.is_err() {
        return Err(MessageErrorDto::new("Failed to load records"));
    }

    Ok(result
        .unwrap()
        .into_iter()
        .map(|message| {
            let core_message = Into::<Message>::into(message);
            From::from(&core_message)
        })
        .collect())
}

pub fn get_topic(table: &MessageTable, id: String) -> Option<FindMessageDto> {
    let message = table.find_topic(id);

    if message.is_none() {
        return None;
    }

    let message: Message = message.unwrap().into();

    Some(FindMessageDto::from(&message))
}
