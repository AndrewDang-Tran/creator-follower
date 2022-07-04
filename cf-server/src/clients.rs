use crate::{
    errors,
    errors::{AnilistServerError, ServiceError},
};
use actix_web::http::StatusCode;
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use serde_json::value::Value;

const ANILIST_GRAPHQL_URL: &str = "https://graphql.anilist.co";

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql_schemas/anilist-schema.graphql",
    query_path = "graphql_schemas/staff-media-query.graphql",
    response_derives = "Serialize,Debug"
)]
pub struct StaffMediaQuery;

#[derive(Clone)]
pub struct AnilistClient {
    pub client: Client,
}

impl AnilistClient {
    pub async fn get_staff_media(
        &self,
        id: i64,
    ) -> Result<staff_media_query::ResponseData, ServiceError> {
        let staff_media_query_variables: staff_media_query::Variables =
            staff_media_query::Variables { id: Some(id) };
        let staff_media_request = StaffMediaQuery::build_query(staff_media_query_variables);

        let res = self
            .client
            .post(ANILIST_GRAPHQL_URL)
            .json(&staff_media_request)
            .send()
            .await
            .unwrap();
        let status_code = &StatusCode::from_u16(res.status().as_u16()).expect("asdasd");

        let response_body: Response<staff_media_query::ResponseData> = res.json().await.unwrap();
        if response_body.errors.is_some() {
            let errors = response_body.errors.ok_or(errors::internal_logic_error(
                "response_body.errors is None after check",
            ))?;

            let first = errors
                .into_iter()
                .nth(0)
                .ok_or(errors::anilist_data_format(
                    "response_body.errors exists but is empty",
                ))?;
            let anilist_error = AnilistServerError {
                message: first.message,
                status_code: *status_code,
            };

            return Err(ServiceError::from(anilist_error));
        }
        response_body
            .data
            .ok_or(errors::anilist_data_format("Data is None"))
    }
}
