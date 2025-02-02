use std::{env, time::Duration};

use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

use crate::{
    helpers::{
        tokenization::Token,
        utils::{self, to_base64},
    },
    models::{
        apis::{self, AppState},
        errors::ApiError,
    },
};

#[get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok().body("/ -> Home Page")
}

#[get("/api")]
async fn get_api_index() -> impl Responder {
    HttpResponse::Ok().body("/api -> Root Api Page")
}

#[get("/api/echo")]
async fn get_api_echo(app_state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let token_header = req.headers().get("Authorization");
    let token_manager = app_state.token_manager.lock().unwrap();

    if let Some(token_header) = token_header {
        match Token::parse(
            token_header.to_str().unwrap(),
            &to_base64(env::var("GLOBAL_JWT_SECRET").unwrap().as_str()),
        ) {
            Ok(tc) => match &token_manager.get_token(&tc.id) {
                Ok(token) => HttpResponse::Ok().json(apis::EchoResponse {
                    status: "Success".to_string(),
                    latency: format!(
                        "{}ms",
                        utils::get_latency(&req.connection_info().host().to_string(), 0)
                            .unwrap_or(Duration::from_secs(0))
                            .as_millis()
                    ),
                    host_id: token.id.clone(),
                }),
                Err(_) => HttpResponse::Unauthorized()
                    .body(ApiError::UnAuthorizedToken(tc.id.clone()).to_string()),
            },
            Err(_) => HttpResponse::Unauthorized().body(
                ApiError::InvalidToken(String::from(token_header.to_str().unwrap())).to_string(),
            ),
        }
    } else {
        HttpResponse::Unauthorized().body(ApiError::MissingBearerToken.to_string())
    }
}
