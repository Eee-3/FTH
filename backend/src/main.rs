use actix_web::{App, HttpServer};
mod api;
#[actix::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(api::http::hello)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
