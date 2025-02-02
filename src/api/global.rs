use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok().body("/ -> Home Page")
}

#[get("/api")]
async fn get_api_index() -> impl Responder {
    HttpResponse::Ok().body("/api -> Root Api Page")
}
