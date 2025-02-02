use std::{
    env,
    sync::{Arc, Mutex},
};

use actix_web::{middleware, web, App, HttpServer};

mod api;
mod classes;
mod helpers;
mod models;

use api::global;
use helpers::tokenization::TokenManager;

fn main() {
    let _ = http_api_server();
}

#[actix_web::main]
async fn http_api_server() -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .app_data(
                web::Data::new(models::apis::AppState {
                    token_manager: Arc::new(Mutex::new(TokenManager::new(
                        env::var("SQLITE_DB_PATH").unwrap(),
                    ))),
                })
                .clone(),
            )
            .service(global::get_index)
            .service(global::get_api_index)
            .service(global::get_api_echo)
            .wrap(middleware::Compress::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
