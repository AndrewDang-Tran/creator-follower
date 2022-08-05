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

struct SearchResult {
    primary_occupations: Vec<String>,
    full_name: String,
    native_name: String,
    image_link: String,
    rss_link: String,
}

#[derive(Template)]
#[template(path = "search_results.html")]
struct SearchResultsTemplate {
    page_info: search_query::SearchQueryStaffPageInfo,
    search_results: Vec<SearchResult>,
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
    let q = match query_params.into_inner().q {
        Some(v) => v,
        None => "".to_string(),
    };
    let client = &shared_data.anilist_client;
    let staff = client
        .search(&q, 50)
        .await?
        .staff
        .ok_or(errors::anilist_data_format("SearchQueryStaff is None"))?;
    let staff_results = staff
        .results
        .ok_or(errors::anilist_data_format(
            "SearchQueryStafffResults is None",
        ))?
        .into_iter()
        .filter_map(|row| row)
        .map(|row| {
            let primary_occupations = row
                .primary_occupations
                .unwrap()
                .into_iter()
                .filter_map(|o| o)
                .map(|o| o.trim().to_string())
                .collect();
            let name = row.name.unwrap();
            let full_name = name.full.unwrap();
            let native_name = name.native.unwrap();
            let image_link = row.image.unwrap().medium.unwrap();
            let rss_link = anilist_staff_link(row.id);

            SearchResult {
                primary_occupations,
                full_name,
                native_name,
                image_link,
                rss_link,
            }
        })
        .collect::<Vec<SearchResult>>();
    let page_info = staff
        .page_info
        .ok_or(errors::anilist_data_format("PageInfo is None"))?;

    let template = SearchResultsTemplate {
        page_info,
        search_results: staff_results,
    };
    template.to_response()
}

fn anilist_staff_link(staff_id: i64) -> String {
    return format!("http://creatorfollower.com/rss/anilist/staff/{staff_id}").to_string();
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(search_results);
}
