use actix_web::{middleware, web, App, HttpServer};
use std::{
    env,
    sync::{Arc, Mutex},
};

mod api;
mod classes;
mod helpers;
mod models;
mod utils;

use api as TrackVaultApi;
use classes::tokens::TokensDatabase;
use models::AppState;

fn main() {
    let _ = http_api_server("127.0.0.1", 8080);
}

#[actix_web::main]
async fn http_api_server(ip_addr: &str, port: u16) -> Result<(), std::io::Error> {
    let app_state = AppState {
        token_manager: Arc::new(Mutex::new(TokensDatabase::new(
            &env::var("SQLITE_DB_PATH").unwrap(),
        ))),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()).clone())
            .service(TrackVaultApi::get_index)
            .service(TrackVaultApi::get_api_index)
            .service(TrackVaultApi::get_api_echo)
            .service(TrackVaultApi::get_api_hosts)
            .wrap(middleware::Compress::default())
    })
    .workers(2)
    .bind((ip_addr, port))?
    .run()
    .await
}
