use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod get {
    use crate::{
        models::Pagination,
        routes::{ApiError, GetState, api::client::servers::_server_::GetServer},
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

        #[serde(default)]
        directory: String,
    }

    #[derive(ToSchema, Serialize)]
    struct Response {
        #[schema(inline)]
        entries: Pagination<wings_api::DirectoryEntry>,
    }

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = inline(Response)),
        (status = BAD_REQUEST, body = ApiError),
        (status = UNAUTHORIZED, body = ApiError),
        (status = NOT_FOUND, body = ApiError),
    ), params(
        (
            "server" = uuid::Uuid,
            description = "The server ID",
            example = "123e4567-e89b-12d3-a456-426614174000",
        ),
        (
            "page" = i64, Query,
            description = "The page number for pagination",
            example = "1",
        ),
        (
            "per_page" = i64, Query,
            description = "The number of items per page",
            example = "10",
        ),
        (
            "directory" = String, Query,
            description = "The directory to list files from",
            example = "/",
        ),
    ))]
    pub async fn route(
        state: GetState,
        mut server: GetServer,
        Query(params): Query<Params>,
    ) -> (StatusCode, axum::Json<serde_json::Value>) {
        if let Err(errors) = crate::utils::validate_data(&params) {
            return (
                StatusCode::BAD_REQUEST,
                axum::Json(ApiError::new_strings_value(errors)),
            );
        }

        if let Err(error) = server.has_permission("files.read") {
            return (
                StatusCode::UNAUTHORIZED,
                axum::Json(ApiError::new_value(&[&error])),
            );
        }

        if server.is_ignored(&params.directory, true) {
            return (
                StatusCode::NOT_FOUND,
                axum::Json(ApiError::new_value(&["directory not found"])),
            );
        }

        let entries = match server
            .node
            .api_client(&state.database)
            .get_servers_server_files_list(
                server.uuid,
                &params.directory,
                server.0.subuser_ignored_files.unwrap_or_default(),
                params.per_page as u64,
                params.page as u64,
            )
            .await
        {
            Ok(data) => data,
            Err((StatusCode::NOT_FOUND, err)) => {
                return (
                    StatusCode::NOT_FOUND,
                    axum::Json(ApiError::new_wings_value(err)),
                );
            }
            Err((_, err)) => {
                tracing::error!(server = %server.0.uuid, "failed to get server files: {:#?}", err);

                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(ApiError::new_value(&["failed to list server files"])),
                );
            }
        };

        (
            StatusCode::OK,
            axum::Json(
                serde_json::to_value(Response {
                    entries: Pagination {
                        total: entries.total as i64,
                        per_page: params.per_page,
                        page: params.page,
                        data: entries.entries,
                    },
                })
                .unwrap(),
            ),
        )
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .with_state(state.clone())
}
