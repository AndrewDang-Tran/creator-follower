use super::super::{errors, errors::ServiceError};
use crate::clients::search_query;
use crate::AppData;
use actix_web::{
    body::BoxBody, get, http::header::ContentType, http::StatusCode, web, HttpRequest,
    HttpResponse, HttpResponseBuilder,
};
use askama::Template;
use serde::Deserialize;

const MAX_ANILIST_STAFF_SEARCH_RESULTS: i64 = 50;

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
    show_name: String,
    image_link: String,
    rss_link: String,
}

struct CompletedSearchInfo {
    num_found_results: i64,
    num_results_on_page: u32,
}

#[derive(Template)]
#[template(path = "search_results.html")]
struct SearchResultsTemplate {
    search_info: CompletedSearchInfo,
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
        .search(&q, MAX_ANILIST_STAFF_SEARCH_RESULTS)
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
                .ok_or(errors::anilist_data_format(
                    "Staff.results.primary_occupations is None",
                ))?
                .into_iter()
                .filter_map(|o| o)
                .map(|o| o.trim().to_string())
                .collect();
            let name = row.name.ok_or(errors::anilist_data_format(
                "Staff.results.name not provided for Staff",
            ))?;
            let staff_name_collection: Vec<Option<String>> = vec![name.full, name.native];
            let show_name: String = staff_name_collection
                .into_iter()
                .filter_map(|n| n)
                .collect::<Vec<String>>()
                .join(", ");
            let image_link = row
                .image
                .ok_or(errors::anilist_data_format("Staff.results.image is None"))?
                .medium
                .ok_or(errors::anilist_data_format(
                    "Staff.results.image.medium is None",
                ))?;
            let rss_link = anilist_staff_link(row.id);

            Ok(SearchResult {
                primary_occupations,
                show_name,
                image_link,
                rss_link,
            })
        })
        .collect::<Result<Vec<SearchResult>, ServiceError>>()?;
    let page_info = staff
        .page_info
        .ok_or(errors::anilist_data_format("PageInfo is None"))?;

    let search_info = CompletedSearchInfo {
        num_found_results: page_info
            .total
            .ok_or(errors::anilist_data_format("page_info.total is None"))?,
        num_results_on_page: u32::try_from(staff_results.len())?,
    };

    let template = SearchResultsTemplate {
        search_info,
        search_results: staff_results,
    };
    template.to_response()
}

fn anilist_staff_link(staff_id: i64) -> String {
    return format!("https://creatorfollower.com/rss/anilist/staff/{staff_id}").to_string();
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(search_results);
}
