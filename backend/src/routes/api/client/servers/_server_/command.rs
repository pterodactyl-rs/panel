use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod post {
    use crate::{
        response::{ApiResponse, ApiResponseResult},
        routes::{
            ApiError, GetState,
            api::client::servers::_server_::{GetServer, GetServerActivityLogger},
        },
    };
    use axum::http::StatusCode;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;
    use validator::Validate;

    #[derive(ToSchema, Validate, Deserialize)]
    pub struct Payload {
        #[validate(length(min = 1, max = 1024))]
        #[schema(min_length = 1, max_length = 1024)]
        command: String,
    }

    #[derive(ToSchema, Serialize)]
    struct Response {}

    #[utoipa::path(post, path = "/", responses(
        (status = OK, body = inline(Response)),
        (status = UNAUTHORIZED, body = ApiError),
    ), params(
        (
            "server" = uuid::Uuid,
            description = "The server ID",
            example = "123e4567-e89b-12d3-a456-426614174000",
        ),
    ), request_body = inline(Payload))]
    pub async fn route(
        state: GetState,
        server: GetServer,
        activity_logger: GetServerActivityLogger,
        axum::Json(data): axum::Json<Payload>,
    ) -> ApiResponseResult {
        if let Err(errors) = crate::utils::validate_data(&data) {
            return ApiResponse::json(ApiError::new_strings_value(errors))
                .with_status(StatusCode::BAD_REQUEST)
                .ok();
        }

        if let Err(error) = server.has_permission("control.console") {
            return ApiResponse::error(&error)
                .with_status(StatusCode::UNAUTHORIZED)
                .ok();
        }

        let request_body = wings_api::servers_server_commands::post::RequestBody {
            commands: vec![data.command],
        };

        match server
            .node
            .api_client(&state.database)
            .post_servers_server_commands(server.uuid, &request_body)
            .await
        {
            Ok(data) => data,
            Err((_, err)) => {
                tracing::error!(server = %server.uuid, "failed to post server command: {:#?}", err);

                return ApiResponse::error("failed to send command to server")
                    .with_status(StatusCode::INTERNAL_SERVER_ERROR)
                    .ok();
            }
        };

        activity_logger
            .log(
                "server:console.command",
                serde_json::json!({
                    "command": request_body.commands[0],
                }),
            )
            .await;

        ApiResponse::json(Response {}).ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(post::route))
        .with_state(state.clone())
}
