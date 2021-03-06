use actix_files;
use actix_web::{get, middleware, web, App, Error, HttpRequest, HttpServer};
use clients::AnilistClient;
use reqwest;

mod clients;
mod errors;
mod routes;

struct AppState {
    anilist_client: AnilistClient,
}

impl AppState {
    fn new() -> Self {
        AppState {
            anilist_client: AnilistClient {
                client: reqwest::Client::new(),
            },
        }
    }
}

type AppData = web::Data<AppState>;

#[get("/")]
async fn index(_req: HttpRequest) -> Result<actix_files::NamedFile, Error> {
    let file = actix_files::NamedFile::open("static/index.html")?;
    Ok(file)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    pretty_env_logger::init();

    let application_state = AppState::new();
    let data = web::Data::new(application_state);
    println!("Starting server on: http://0.0.0.0:8080");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(data.clone())
            .configure(routes::init_health_routes)
            .configure(routes::init_anilist_routes)
            .service(index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
