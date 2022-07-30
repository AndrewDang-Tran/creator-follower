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

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: Option<String>,
}

#[derive(Template)]
#[template(path = "search_results.html")]
struct SearchResultsTemplate {}

#[get("/search")]
async fn search_results(
    _req: HttpRequest,
    shared_data: AppData,
    query_params: web::Query<SearchQuery>,
) -> Result<HttpResponse<BoxBody>, ServiceError> {
    let template = SearchResultsTemplate {};

    template.to_response()
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(search_results);
}
