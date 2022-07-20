use super::super::{errors, errors::ServiceError};
use crate::clients::staff_media_query::{
    StaffMediaQueryStaffStaffMediaEdges, StaffMediaQueryStaffStaffMediaNodes,
};
use crate::errors::internal_logic_error;
use crate::AppData;
use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder};
use chrono::naive::NaiveDate;
use rss::{Channel, ChannelBuilder, Image, ImageBuilder, Item, ItemBuilder};
use std::{iter, iter::Zip};

const RSS_2_SPECIFICATION_URL: &str = "https://validator.w3.org/feed/docs/rss2.html";
const NO_STAFF_DESCRIPTION: &str = "No description provided by Anilist for this staff.";
const STAFF_NONE: &str = "Staff is None";
const STAFF_MEDIA_BATCH_SIZE: i64 = 25;

struct AnilistMedia {
    role: String,
    media: StaffMediaQueryStaffStaffMediaNodes,
}

#[get("/rss/anilist/staff/{anilist_id}")]
async fn get_anilist_staff_rss_feed(
    path: web::Path<i64>,
    data: AppData,
) -> Result<impl Responder, ServiceError> {
    let mut current_page: i64 = 1;

    let id: i64 = path.into_inner();
    let client = &data.anilist_client;
    let staff = client
        .get_staff_media(id, STAFF_MEDIA_BATCH_SIZE, current_page)
        .await?
        .staff
        .ok_or(errors::anilist_data_format(STAFF_NONE))?;
    let staff_media = staff
        .staff_media
        .ok_or(errors::anilist_data_format("Staff.staffMedia is None"))?;

    let mut roles = staff_media.edges.ok_or(errors::anilist_data_format(
        "Staff.staffMedia.edges is None",
    ))?;
    let mut media = staff_media.nodes.ok_or(errors::anilist_data_format(
        "Staff.staffMedia.nodes is None",
    ))?;

    let anilist_staff_name = staff
        .name
        .ok_or(errors::anilist_data_format("Staff.name is None"))?;
    let staff_name_collection: Vec<Option<String>> =
        vec![anilist_staff_name.full, anilist_staff_name.native];
    let staff_name: String = staff_name_collection
        .into_iter()
        .filter(|name| name.is_some())
        .map(|name| {
            name.ok_or(errors::internal_logic_error(
                "name cannot be None after is_some() check",
            ))
        })
        .collect::<Result<Vec<String>, ServiceError>>()?
        .join(", ");

    let mut media_in_page = media.len();
    let mut zipped_role_media = vec![roles.into_iter().zip(media.into_iter())];

    current_page += 1;
    while media_in_page == (STAFF_MEDIA_BATCH_SIZE as usize) {
        let staff_media_page = client
            .get_staff_media(id, STAFF_MEDIA_BATCH_SIZE, current_page)
            .await?
            .staff
            .ok_or(errors::anilist_data_format(STAFF_NONE))?
            .staff_media
            .ok_or(errors::anilist_data_format("Staff.staffMedia is None"))?;
        roles = staff_media_page.edges.ok_or(errors::anilist_data_format(
            "Staff.staffMedia.edges is None",
        ))?;
        media = staff_media_page.nodes.ok_or(errors::anilist_data_format(
            "Staff.staffMedia.nodes is None",
        ))?;
        media_in_page = media.len();
        current_page += 1;

        zipped_role_media.push(roles.into_iter().zip(media.into_iter()));
    }

    let mut staff_channel_items: Vec<Item> = zipped_role_media
        .into_iter()
        .flatten()
        .map(|(r, m)| {
            let has_role = r.is_some();
            let has_media = m.is_some();
            if !has_media || !has_role {
                return Ok(None);
            }
            let start_date = m
                .as_ref()
                .ok_or(errors::internal_logic_error("Staff.staffMedia[] is None"))?
                .start_date
                .as_ref()
                .ok_or(errors::anilist_data_format("Staff.staffMedia.startDate"))?;
            let has_start_date =
                start_date.year.is_some() || start_date.month.is_some() || start_date.day.is_some();
            let string_role = r
                .ok_or(errors::internal_logic_error(
                    "None role after is_some check",
                ))?
                .staff_role
                .ok_or(errors::anilist_data_format("Staff.staffMedia.edges"))?;
            if has_start_date {
                Ok(Some(AnilistMedia {
                    role: string_role,
                    media: m.ok_or(errors::internal_logic_error(
                        "Staff.staffMedia.nodes[] is None after is_some check",
                    ))?,
                }))
            } else {
                Ok(None)
            }
        })
        .collect::<Result<Vec<Option<AnilistMedia>>, ServiceError>>()?
        .into_iter()
        .filter(|o_m| o_m.is_some())
        .map(|o| {
            let anilist_media =
                o.ok_or(errors::internal_logic_error("Logically cannot be None"))?;
            let m = anilist_media.media;
            let mut title: String = "Anilist has no title".to_string();
            if let Some(t) = m.title {
                let mut english = t.romaji;
                if t.english.is_some() {
                    english = t.english;
                }

                let anime_name_collection: Vec<Option<String>> = vec![english, t.native];
                title = anime_name_collection
                    .into_iter()
                    .filter(|name| name.is_some())
                    .map(|name| {
                        name.ok_or(errors::internal_logic_error(
                            "name cannot be None after check",
                        ))
                    })
                    .collect::<Result<Vec<String>, ServiceError>>()?
                    .join(", ");
                title = staff_name.clone() + " as " + &anilist_media.role + " on " + &title;
            }

            let start_date = m.start_date.as_ref().ok_or(errors::internal_logic_error(
                "Staff.staffMedia[].startDate is None after check",
            ))?;
            let year = start_date.year.ok_or(errors::internal_logic_error(
                "Staff.staffMedia[].startDate.year is None after check",
            ))?;
            let month = start_date.month.ok_or(errors::internal_logic_error(
                "Staff.staffMedia[].startDate.month is None after check",
            ))?;
            let day = start_date.day.ok_or(errors::internal_logic_error(
                "Staff.staffMedia[].startDate.day is None after check",
            ))?;
            let naive_date = NaiveDate::from_ymd(year as i32, month as u32, day as u32);

            Ok(ItemBuilder::default()
                .title(Some(title))
                .link(m.site_url)
                .description(m.description)
                .pub_date(Some(naive_date.to_string()))
                .build())
        })
        .collect::<Result<Vec<Item>, ServiceError>>()?;

    staff_channel_items.sort_by(|a, b| {
        let b_date = b.pub_date().unwrap();
        let a_date = a.pub_date().unwrap();
        b_date.cmp(a_date)
    });

    let site_url = staff
        .site_url
        .ok_or(errors::anilist_data_format("Staff.siteUrl is None"))?;
    let image_url = staff
        .image
        .ok_or(errors::anilist_data_format("Staff.image is None"))?
        .large
        .ok_or(errors::anilist_data_format("Staff.image.large is None"))?;
    let rss_image: Image = ImageBuilder::default()
        .title(&staff_name)
        .link(&site_url)
        .url(&image_url)
        .build();
    let mut staff_description = NO_STAFF_DESCRIPTION.to_string();
    if staff.description.is_some() {
        staff_description = staff.description.ok_or(errors::internal_logic_error(
            "Staff.description cannot be None",
        ))?;
    }

    let staff_channel: Channel = ChannelBuilder::default()
        .title(&staff_name)
        .link(&site_url)
        .description(&staff_description)
        .image(rss_image)
        .docs(RSS_2_SPECIFICATION_URL.to_string())
        .items(staff_channel_items)
        .build();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::xml())
        .body(staff_channel.to_string()))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_anilist_staff_rss_feed);
}
