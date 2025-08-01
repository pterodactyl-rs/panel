use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod _pull_;

mod get {
    use crate::{
        response::{ApiResponse, ApiResponseResult},
        routes::{ApiError, GetState, api::client::servers::_server_::GetServer},
    };
    use axum::http::StatusCode;
    use serde::Serialize;
    use utoipa::ToSchema;

    #[derive(ToSchema, Serialize)]
    struct Response {
        pulls: Vec<wings_api::Download>,
    }

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = inline(Response)),
        (status = UNAUTHORIZED, body = ApiError),
    ), params(
        (
            "server" = uuid::Uuid,
            description = "The server ID",
            example = "123e4567-e89b-12d3-a456-426614174000",
        ),
    ))]
    pub async fn route(state: GetState, server: GetServer) -> ApiResponseResult {
        if let Err(error) = server.has_permission("files.read") {
            return ApiResponse::error(&error)
                .with_status(StatusCode::UNAUTHORIZED)
                .ok();
        }

        let pulls = match server
            .node
            .api_client(&state.database)
            .get_servers_server_files_pull(server.uuid)
            .await
        {
            Ok(data) => data.downloads,
            Err((StatusCode::EXPECTATION_FAILED, err)) => {
                return ApiResponse::json(ApiError::new_wings_value(err))
                    .with_status(StatusCode::EXPECTATION_FAILED)
                    .ok();
            }
            Err((_, err)) => {
                tracing::error!(server = %server.uuid, "failed to list server file pulls: {:#?}", err);

                return ApiResponse::error("failed to list server file pulls")
                    .with_status(StatusCode::INTERNAL_SERVER_ERROR)
                    .ok();
            }
        };

        ApiResponse::json(Response { pulls }).ok()
    }
}

mod post {
    use crate::routes::{
        ApiError, GetState,
        api::client::servers::_server_::{GetServer, GetServerActivityLogger},
    };
    use axum::http::StatusCode;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;
    use validator::Validate;

    #[derive(ToSchema, Validate, Deserialize)]
    pub struct Payload {
        #[serde(default)]
        #[schema(default = "/")]
        root: String,

        #[validate(url)]
        #[schema(format = "uri")]
        url: String,
        name: Option<String>,

        #[serde(default)]
        #[schema(default = false)]
        use_header: bool,
        #[serde(default)]
        #[schema(default = false)]
        foreground: bool,
    }

    #[derive(ToSchema, Serialize)]
    struct Response {
        identifier: uuid::Uuid,
    }

    #[utoipa::path(post, path = "/", responses(
        (status = OK, body = inline(Response)),
        (status = UNAUTHORIZED, body = ApiError),
        (status = EXPECTATION_FAILED, body = ApiError),
    ), params(
        (
            "server" = uuid::Uuid,
            description = "The server ID",
            example = "123e4567-e89b-12d3-a456-426614174000",
        ),
    ), request_body = inline(Payload))]
    pub async fn route(
        state: GetState,
        mut server: GetServer,
        activity_logger: GetServerActivityLogger,
        axum::Json(data): axum::Json<Payload>,
    ) -> (StatusCode, axum::Json<serde_json::Value>) {
        if let Err(error) = server.has_permission("files.create") {
            return (
                StatusCode::UNAUTHORIZED,
                axum::Json(ApiError::new_value(&[&error])),
            );
        }

        if let Some(name) = &data.name {
            if server.is_ignored(name, false) {
                return (
                    StatusCode::NOT_FOUND,
                    axum::Json(ApiError::new_value(&["root directory not found"])),
                );
            }
        }

        let request_body = wings_api::servers_server_files_pull::post::RequestBody {
            root: data.root,
            url: data.url,
            file_name: data.name,
            use_header: data.use_header,
            foreground: data.foreground,
        };

        let data = match server
            .node
            .api_client(&state.database)
            .post_servers_server_files_pull(server.uuid, &request_body)
            .await
        {
            Ok(data) => data,
            Err((StatusCode::NOT_FOUND, err)) => {
                return (
                    StatusCode::NOT_FOUND,
                    axum::Json(ApiError::new_wings_value(err)),
                );
            }
            Err((StatusCode::EXPECTATION_FAILED, err)) => {
                return (
                    StatusCode::EXPECTATION_FAILED,
                    axum::Json(ApiError::new_wings_value(err)),
                );
            }
            Err((_, err)) => {
                tracing::error!(server = %server.uuid, "failed to pull server file: {:#?}", err);

                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(ApiError::new_value(&["failed to pull server file"])),
                );
            }
        };

        activity_logger
            .log(
                "server:file.pull",
                serde_json::json!({
                    "identifier": data.identifier,
                    "directory": request_body.root,
                    "url": request_body.url,
                }),
            )
            .await;

        (
            StatusCode::OK,
            axum::Json(
                serde_json::to_value(Response {
                    identifier: data.identifier,
                })
                .unwrap(),
            ),
        )
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .routes(routes!(post::route))
        .nest("/{pull}", _pull_::router(state))
        .with_state(state.clone())
}
