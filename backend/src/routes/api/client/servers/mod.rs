use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod _server_;

mod get {
    use crate::{
        models::{Pagination, server::Server},
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState, api::client::GetUser},
    };
    use axum::{extract::Query, http::StatusCode};
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;
    use validator::Validate;

    #[derive(ToSchema, Validate, Deserialize)]
    pub struct Params {
        #[validate(range(min = 1))]
        #[serde(default = "Pagination::default_page")]
        pub page: i64,
        #[validate(range(min = 1, max = 100))]
        #[serde(default = "Pagination::default_per_page")]
        pub per_page: i64,
        #[validate(length(min = 1, max = 100))]
        #[serde(
            default,
            deserialize_with = "crate::deserialize::deserialize_string_option"
        )]
        pub search: Option<String>,

        #[serde(default)]
        other: bool,
    }

    #[derive(ToSchema, Serialize)]
    struct Response {
        #[schema(inline)]
        servers: Pagination<crate::models::server::ApiServer>,
    }

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = inline(Response)),
    ), params(
        (
            "page" = i64, Query,
            description = "The page number",
            example = "1",
        ),
        (
            "per_page" = i64, Query,
            description = "The number of items per page",
            example = "10",
        ),
        (
            "search" = Option<String>, Query,
            description = "Search term for items",
        ),
        (
            "other" = bool, Query,
            description = "If true, returns servers not owned by the user (admin only)",
            example = "false",
        ),
    ))]
    pub async fn route(
        state: GetState,
        user: GetUser,
        Query(params): Query<Params>,
    ) -> ApiResponseResult {
        if let Err(errors) = crate::utils::validate_data(&params) {
            return ApiResponse::json(ApiError::new_strings_value(errors))
                .with_status(StatusCode::BAD_REQUEST)
                .ok();
        }

        let servers = if params.other && user.admin {
            Server::by_not_user_id_with_pagination(
                &state.database,
                user.id,
                params.page,
                params.per_page,
                params.search.as_deref(),
            )
            .await
        } else {
            Server::by_user_id_with_pagination(
                &state.database,
                user.id,
                params.page,
                params.per_page,
                params.search.as_deref(),
            )
            .await
        }?;

        ApiResponse::json(Response {
            servers: Pagination {
                total: servers.total,
                per_page: servers.per_page,
                page: servers.page,
                data: servers
                    .data
                    .into_iter()
                    .map(|server| server.into_api_object(&user))
                    .collect(),
            },
        })
        .ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .nest("/{server}", _server_::router(state))
        .with_state(state.clone())
}
