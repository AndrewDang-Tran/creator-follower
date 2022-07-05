use super::super::{errors, errors::ServiceError};
use crate::clients::staff_media_query::StaffMediaQueryStaffStaffMediaNodes;
use crate::AppData;
use actix_web::{get, http::header::ContentType, web, HttpResponse, Responder};
use chrono::naive::NaiveDate;
use rss::{Channel, ChannelBuilder, Image, ImageBuilder, Item, ItemBuilder};

const RSS_2_SPECIFICATION_URL: &str = "https://validator.w3.org/feed/docs/rss2.html";
const NO_STAFF_DESCRIPTION: &str = "No description provided by Anilist for this staff.";
const STAFF_NONE: &str = "Staff is None";

#[get("/rss/anilist/staff/{anilist_id}")]
async fn get_anilist_staff_rss_feed(
    path: web::Path<i64>,
    data: AppData,
) -> Result<impl Responder, ServiceError> {
    let id: i64 = path.into_inner();
    let client = &data.anilist_client;
    let staff = client
        .get_staff_media(id)
        .await?
        .staff
        .ok_or(errors::anilist_data_format(STAFF_NONE))?;
    let media = staff
        .staff_media
        .ok_or(errors::anilist_data_format("Staff.staffMedia is None"))?
        .nodes
        .ok_or(errors::anilist_data_format(
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

    let staff_channel_items: Vec<Item> = media
        .into_iter()
        .map(|m| {
            let has_media = m.is_some();
            if !has_media {
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
            if has_start_date {
                Ok(m)
            } else {
                Ok(None)
            }
        })
        .collect::<Result<Vec<Option<StaffMediaQueryStaffStaffMediaNodes>>, ServiceError>>()?
        .into_iter()
        .filter(|o_m| o_m.is_some())
        .map(|o| {
            let m = o.ok_or(errors::internal_logic_error("Logically cannot be None"))?;
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
