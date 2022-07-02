use std::sync::Arc;
use actix_web::{App, HttpServer, middleware, web};

mod routes;
mod errors;

struct AppState {}

impl AppState {
    async fn new() -> Arc<Self> {
        let application_state = AppState {};

        Arc::new(application_state)
    }
}

type AppData = web::Data<Arc<AppState>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    pretty_env_logger::init();

    let application_state = AppState::new().await;

    let data = web::Data::new(application_state);
    println!("Starting server on: http://127.0.0.1");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(data.clone())
            .configure(routes::init_health_routes)
            .configure(routes::init_anilist_routes)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
