use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::orm::tables::message::MessageTable;
use crate::use_cases::dto::MessageErrorDto;
use crate::use_cases::{
    get_all_message, get_all_topics, get_message, get_topic, post_message, post_topic,
};

#[derive(Deserialize)]
pub struct IdPath(String);

#[derive(Deserialize, Clone)]
pub struct NewPost {
    text: String,
    reply_to: String,
}

#[derive(Deserialize, Clone)]
pub struct NewTopic {
    text: String,
}

#[get("/posts")]
pub async fn get_posts(table: web::Data<MessageTable>) -> actix_web::Result<impl Responder> {
    let result = web::block(move || get_all_message(&table)).await?;

    match result {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(err) => Ok(HttpResponse::InternalServerError().json(err)),
    }
}

#[get("/posts/{id}")]
pub async fn get_post(
    id: web::Path<IdPath>,
    table: web::Data<MessageTable>,
) -> actix_web::Result<impl Responder> {
    let result = web::block(move || get_message(&table, id.0.clone())).await?;

    match result {
        Some(res) => Ok(HttpResponse::Ok().json(res)),
        None => Ok(HttpResponse::NotFound().json(MessageErrorDto::new("Not found"))),
    }
}

#[get("/topics")]
pub async fn get_topics(table: web::Data<MessageTable>) -> actix_web::Result<impl Responder> {
    let result = web::block(move || get_all_topics(&table)).await?;

    match result {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(err) => Ok(HttpResponse::InternalServerError().json(err)),
    }
}

#[get("/topics/{id}")]
pub async fn get_topic_route(
    id: web::Path<IdPath>,
    table: web::Data<MessageTable>,
) -> actix_web::Result<impl Responder> {
    let result = web::block(move || get_topic(&table, id.0.clone())).await?;

    match result {
        Some(res) => Ok(HttpResponse::Ok().json(res)),
        None => Ok(HttpResponse::NotFound().json(MessageErrorDto::new("Not found"))),
    }
}

#[post("/posts")]
pub async fn new_post(
    data: web::Json<NewPost>,
    table: web::Data<MessageTable>,
) -> actix_web::Result<impl Responder> {
    let result = web::block(move || {
        let data = data.clone();
        post_message(&table, data.text, data.reply_to)
    })
    .await?;

    match result {
        Ok(res) => match res.first() {
            Some(first) => Ok(HttpResponse::Ok().json(first)),
            None => Ok(HttpResponse::InternalServerError()
                .json(MessageErrorDto::new("Failed to save post"))),
        },
        Err(err) => Ok(HttpResponse::InternalServerError().json(err)),
    }
}

#[post("/topics")]
pub async fn new_topic(
    data: web::Json<NewTopic>,
    table: web::Data<MessageTable>,
) -> actix_web::Result<impl Responder> {
    let result = web::block(move || {
        let data = data.clone();
        post_topic(&table, data.text)
    })
    .await?;

    match result {
        Ok(res) => match res.first() {
            Some(first) => Ok(HttpResponse::Ok().json(first)),
            None => Ok(HttpResponse::InternalServerError()
                .json(MessageErrorDto::new("Failed to save post"))),
        },
        Err(err) => Ok(HttpResponse::InternalServerError().json(err)),
    }
}
