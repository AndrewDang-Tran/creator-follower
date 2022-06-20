use actix_web::{get, Responder, web};
use serde::{Deserialize, Serialize};
use crate::AppData;
use rss::Channel;

#[derive(Serialize)]
pub struct AnilistMediaResponse {
    pub media_list: Vec<AnilistMedia>
}

pub struct AnilistStaff

pub struct AnilistMedia {
    pub id: i64,
    #[serde(rename(serialize="type")]
    pub media_type: String,
    pub title: AnilistTitle,
    pub description: String,
    pub siteUrl: String,
    pub status: String
}

pub struct AnilistTitle {
    english: Option<String>,
    romaji: Option<String>,
    native: Option<String>
}

pub struct AnilistCoverImage {
    medium: String
}

#[get("/anilist/rss/{anilist_id")]
async fn get_anilist_rss_feed(path: web::Path<i64>,
                                data: AppData) -> impl Responder {
    let anilist_id: i64 = path.into_inner();
    let json_body: String = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    let AniListMedia



}

#[get("/arguments/{argument_id}")]
async fn get_argument(path: web::Path<i64>,
                      data: AppData) -> impl Responder {
    let argument_id = path.into_inner();
    let argument_result = internal_get_argument(&data, argument_id).await;
    match argument_result {
        Ok(argument) => {
            HttpResponse::Ok().json(argument)
        },
        Err(_e) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}
