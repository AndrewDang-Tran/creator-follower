use super::super::{errors, errors::ServiceError};
use crate::clients::search_query;
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
struct SearchResultsTemplate {
    page_info: search_query::SearchQueryStaffPageInfo,
    //results: Vec<search_query::SearchQueryStaffResults>,
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
    let client = &shared_data.anilist_client;
    let staff = client
        .search("Naoko Yamada", 50)
        .await?
        .staff
        .ok_or(errors::anilist_data_format("SearchQueryStaff is None"))?;
    let staff_results = staff.results.ok_or(errors::anilist_data_format(
        "SearchQueryStafffResults is None",
    ))?;
    let page_info = staff
        .page_info
        .ok_or(errors::anilist_data_format("PageInfo is None"))?;

    let template = SearchResultsTemplate { page_info };
    template.to_response()
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(search_results);
}
