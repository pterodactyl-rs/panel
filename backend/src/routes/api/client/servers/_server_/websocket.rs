use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod get {
    use crate::{
        jwt::BasePayload,
        response::{ApiResponse, ApiResponseResult},
        routes::{
            GetState,
            api::client::{GetUser, servers::_server_::GetServer},
        },
    };
    use serde::Serialize;
    use utoipa::ToSchema;

    #[derive(ToSchema, Serialize)]
    struct Response {
        token: String,
        #[schema(format = "uri")]
        url: String,
    }

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = inline(Response)),
    ), params(
        (
            "server" = uuid::Uuid,
            description = "The server ID",
            example = "123e4567-e89b-12d3-a456-426614174000",
        ),
    ))]
    pub async fn route(state: GetState, user: GetUser, server: GetServer) -> ApiResponseResult {
        #[derive(Serialize)]
        struct WebsocketJwt<'a> {
            #[serde(flatten)]
            base: BasePayload,

            user_uuid: uuid::Uuid,
            server_uuid: uuid::Uuid,
            permissions: Vec<&'a str>,
        }

        let token = server.node.create_jwt(
            &state.database,
            &state.jwt,
            &WebsocketJwt {
                base: BasePayload {
                    issuer: "panel".into(),
                    subject: None,
                    audience: Vec::new(),
                    expiration_time: Some(chrono::Utc::now().timestamp() + 600),
                    not_before: None,
                    issued_at: Some(chrono::Utc::now().timestamp()),
                    jwt_id: user.id.to_string(),
                },
                user_uuid: user.to_uuid(),
                server_uuid: server.uuid,
                permissions: server.wings_permissions(&user),
            },
        )?;

        let mut url = server.node.public_url();
        url.set_path(&format!("/api/servers/{}/ws", server.uuid));
        if url.scheme() == "http" {
            url.set_scheme("ws").unwrap();
        } else if url.scheme() == "https" {
            url.set_scheme("wss").unwrap();
        }

        ApiResponse::json(Response {
            token,
            url: url.to_string(),
        })
        .ok()
    }
}

pub fn router(state: &State) -> OpenApiRouter<State> {
    OpenApiRouter::new()
        .routes(routes!(get::route))
        .with_state(state.clone())
}
