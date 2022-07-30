mod anilist_routes;
mod health_routes;
mod page_routes;

pub use anilist_routes::init as init_anilist_routes;
pub use health_routes::init as init_health_routes;
pub use page_routes::init as init_page_routes;
