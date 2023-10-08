mod core;
mod orm;
mod routes;
mod test;
mod use_cases;

use std::env;
use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use dotenvy::dotenv;

use orm::db::Db;
use orm::tables::message::MessageTable;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let host = env::var("APP_HOST").unwrap_or("localhost".into());
    let port: u16 = env::var("APP_PORT")
        .unwrap_or("8080".into())
        .parse()
        .expect("APP_PORT must be a positive number");

    println!("Used host: {}:{}", host, port);

    let db = Arc::new(Db::new(&database_url));
    let message_table = Arc::new(MessageTable::new(db.clone()));

    HttpServer::new(move || {
        let cors = Cors::default()
              .allow_any_origin()
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec!["Content-Type"])
              .max_age(3600);

        App::new()
            .wrap(cors)
            .service(routes::get_posts)
            .service(routes::get_post)
            .service(routes::get_topics)
            .service(routes::get_topic_route)
            .service(routes::new_post)
            .service(routes::new_topic)
            .app_data(web::Data::from(message_table.clone()))
    })
    .bind((host, port))?
    .run()
    .await
}
