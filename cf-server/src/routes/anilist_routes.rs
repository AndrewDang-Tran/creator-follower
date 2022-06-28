use actix_web::{get, Responder, web, HttpResponse};
use crate::AppData;
use graphql_client::{GraphQLQuery, Response};
use serde::Serialize;
use reqwest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql_schemas/anilist-schema.graphql",
    query_path = "graphql_schemas/staff-media-query.graphql",
    response_derives = "Serialize,Debug"
)]
pub struct StaffMediaQuery;

#[get("/anilist/rss/{anilist_id}")]
async fn get_anilist_rss_feed(path: web::Path<i64>,
                              data: AppData) -> impl Responder {
    let id: i64 = path.into_inner();
    let staff_media_query_variables: staff_media_query::Variables = staff_media_query::Variables {
        id: Some(id)
    };
    let staff_media_request = StaffMediaQuery::build_query(staff_media_query_variables);
    let client = reqwest::Client::new();

    let mut res = client.post("https://graphql.anilist.co").json(&staff_media_request).send().await.unwrap();
    let response_body: Response<staff_media_query::ResponseData> = res.json().await.unwrap();
    println!("{:#?}", response_body);
    web::Json(response_body)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_anilist_rss_feed);
}
