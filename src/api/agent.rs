use std::{env, time::Duration};

use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use crate::{
    classes::{LogManager, TokenMetadata},
    models::{ApiEchoResponse, ApiError, AppState},
    utils::{get_latency, to_base64},
};

#[get("/api/echo")]
async fn get_api_echo(app_state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let token_header = req.headers().get("Authorization");
    let token_manager = app_state.token_database.lock().unwrap();

    if let Some(token_header) = token_header {
        match TokenMetadata::parse(
            token_header.to_str().unwrap(),
            &to_base64(env::var("GLOBAL_JWT_SECRET").unwrap().as_str()),
        ) {
            Ok(tc) => match &token_manager.get_token(&tc.id) {
                Ok(token) => HttpResponse::Ok().json(ApiEchoResponse {
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
                        .body(ApiError::UnAuthorizedToken(tc.id.clone()).to_string())
                }
            },
            Err(_) => HttpResponse::Unauthorized().body(
                ApiError::InvalidToken(String::from(token_header.to_str().unwrap())).to_string(),
            ),
        }
    } else {
        HttpResponse::Unauthorized().body(ApiError::MissingBearerToken.to_string())
    }
}

#[get("/api/agents")]
async fn get_api_agents(app_state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let token_header = req.headers().get("Authorization");
    let token_manager = app_state.token_database.lock().unwrap();
    let super_token = env::var("notSoSecureToken").unwrap();

    if let Some(token_header) = token_header {
        if &super_token == token_header.to_str().unwrap() {
            match token_manager.get_tokens() {
                Ok(tokens) => HttpResponse::Ok().json(tokens),
                Err(err) => {
                    LogManager::eprint(Some("API-Error"), &err);
                    HttpResponse::InternalServerError()
                        .body(ApiError::InternalServerError(err.to_string()).to_string())
                }
            }
        } else {
            HttpResponse::Unauthorized().body(
                ApiError::UnAuthorizedToken(String::from(token_header.to_str().unwrap()))
                    .to_string(),
            )
        }
    } else {
        HttpResponse::Unauthorized().body(ApiError::MissingBearerToken.to_string())
    }
}
