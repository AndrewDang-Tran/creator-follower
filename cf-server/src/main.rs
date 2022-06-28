use std::sync::Arc;
use actix_web::{App, HttpServer, middleware, web};
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

mod routes;

struct AppState {
    pub db_connection_pool: Pool<Sqlite>
}

impl AppState {
    async fn new() -> Arc<Self> {
        let pool = SqlitePoolOptions::new().max_connections(5)
                                           .connect("sqlite://creator-follower.db")
                                           .await
                                           .expect("Failed to create DB connection pool");

        let application_state = AppState {
            db_connection_pool: pool
        };

        Arc::new(application_state)
    }
}

type AppData = web::Data<Arc<AppState>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    pretty_env_logger::init();

    let application_state = AppState::new().await;
    sqlx::migrate!("./migrations").run(&application_state.db_connection_pool)
                                  .await
                                  .expect("Failed to run sqlx migrations");

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
