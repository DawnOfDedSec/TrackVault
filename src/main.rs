use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
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
use classes::{tokens::TokensDatabase, LogManager};
use models::{ApiError, AppState};

fn main() {
    dotenv().ok();
    let _ = api_server("127.0.0.1", 8080);
}

#[actix_web::main]
async fn api_server(ip_addr: &str, port: u16) -> Result<(), ApiError> {
    LogManager::print(
        Some("API"),
        "Initializing API Server with static configuration",
    );

    let token_database = TokensDatabase::new(&env::var("SQLITE_DB_PATH").unwrap());

    match token_database.init() {
        Ok(_) => (),
        Err(err) => {
            LogManager::eprint(Some("API"), &err);
            return Err(ApiError::InternalServerError(err.to_string()));
        }
    }

    let app_state = AppState {
        token_database: Arc::new(Mutex::new(token_database)),
    };

    match HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()).clone())
            .service(TrackVaultApi::get_index)
            .service(TrackVaultApi::get_api_index)
            .service(TrackVaultApi::get_api_echo)
            .service(TrackVaultApi::get_api_agents)
            .wrap(middleware::Compress::default())
    })
    .workers(1)
    .bind((ip_addr, port))
    {
        Ok(server) => {
            LogManager::print(
                Some("API"),
                &format!("Server listening on http://{}:{}", &ip_addr, &port),
            );

            match server.run().await {
                Ok(_) => {
                    LogManager::print(
                        Some("API"),
                        &format!("Server stopped listening on http://{}:{}", &ip_addr, &port),
                    );
                    Ok(())
                }
                Err(err) => {
                    LogManager::eprint(Some("API"), &err);
                    Err(ApiError::RuntimeServerError(err))
                }
            }
        }
        Err(err) => {
            LogManager::eprint(Some("API"), &err);
            Err(ApiError::RuntimeServerError(err))
        }
    }
}
