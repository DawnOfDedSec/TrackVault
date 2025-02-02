use std::{env, time::Duration};

use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use crate::{
    classes::LogManager,
    models::{AgentResponse, ApiError, AppState, EchoResponse},
    utils::get_latency,
};

#[get("/api/echo")]
async fn get_api_echo(app_state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let token_header = req.headers().get("Authorization");
    let token_manager = app_state.token_database.lock().unwrap();

    if let Some(token_header) = token_header {
        let token = token_header.to_str().unwrap();

        match token_manager.get(Some(token), None) {
            Ok(token) => HttpResponse::Ok().json(EchoResponse {
                status: "Success".to_string(),
                latency: format!(
                    "{}ms",
                    get_latency(&req.connection_info().host().to_string(), 0)
                        .unwrap_or(Duration::from_secs(0))
                        .as_millis()
                ),
                host_id: token.id.clone(),
            }),
            Err(err) => {
                LogManager::eprint(Some("API-Error"), &err);
                HttpResponse::Unauthorized()
                    .body(ApiError::UnAuthorizedToken(String::from(token)).to_string())
            }
        }
    } else {
        HttpResponse::Unauthorized().body(ApiError::MissingBearerToken.to_string())
    }
}

#[get("/api/agents")]
async fn get_api_agents(app_state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let root_token_header = req.headers().get("Root-Authorization");
    let token_manager = app_state.token_database.lock().unwrap();
    let env_super_token = env::var("GLOBAL_SUPER_TOKEN").unwrap();

    if let Some(root_token_header) = root_token_header {
        if &env_super_token == root_token_header.to_str().unwrap() {
            match token_manager.get_all() {
                Ok(tokens) => HttpResponse::Ok().json(AgentResponse {
                    count: tokens.len() as u16,
                    agents: tokens,
                }),
                Err(err) => {
                    LogManager::eprint(Some("API-Error"), &err);
                    HttpResponse::InternalServerError()
                        .body(ApiError::InternalServerError(err.to_string()).to_string())
                }
            }
        } else {
            HttpResponse::Unauthorized().body(
                ApiError::UnAuthorizedToken(String::from(root_token_header.to_str().unwrap()))
                    .to_string(),
            )
        }
    } else {
        HttpResponse::Unauthorized().body(ApiError::MissingBearerToken.to_string())
    }
}
