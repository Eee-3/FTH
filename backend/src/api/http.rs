use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

#[get("/ping")]
pub async fn hello() -> impl Responder {
    web::Json(json!({"message": "pong"}))
}
