use super::super::errors::ServiceError;
use crate::AppData;
use actix_web::{
    body::BoxBody, get, http::header::ContentType, http::StatusCode, web, HttpRequest,
    HttpResponse, HttpResponseBuilder,
};
use askama::Template;
use serde::Deserialize;

pub trait TemplateToResponse {
    fn to_response(&self) -> Result<HttpResponse<BoxBody>, ServiceError>;
}

impl<T: Template> TemplateToResponse for T {
    fn to_response(&self) -> Result<HttpResponse<BoxBody>, ServiceError> {
        match self.render() {
            Ok(buffer) => Ok(HttpResponseBuilder::new(StatusCode::OK)
                .content_type(ContentType::html())
                .body(buffer)),
            Err(err) => Err(ServiceError::AskamaError(err)),
        }
    }
}

#[derive(Template)]
#[template(path = "base.html")]
struct BaseTemplate {}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

#[get("/")]
async fn index(_req: HttpRequest) -> Result<HttpResponse<BoxBody>, ServiceError> {
    let template = IndexTemplate {};

    template.to_response()
}

#[derive(Template)]
#[template(path = "search_results.html")]
struct SearchResultsTemplate<'a> {
    _parent: &'a BaseTemplate,
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: Option<String>,
}

#[get("/search")]
async fn search_results(
    _req: HttpRequest,
    shared_data: AppData,
    query_params: web::Query<SearchQuery>,
) -> Result<HttpResponse<BoxBody>, ServiceError> {
    let template = SearchResultsTemplate {
        _parent: &BaseTemplate {},
    };

    template.to_response()
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(search_results);
}
