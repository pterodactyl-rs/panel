use super::State;
use utoipa_axum::{router::OpenApiRouter, routes};

mod _egg_;
mod import;

mod get {
    use crate::{
        models::{Pagination, PaginationParamsWithSearch, nest_egg::NestEgg},
        routes::{ApiError, GetState, api::admin::nests::_nest_::GetNest},
    };
    use axum::{extract::Query, http::StatusCode};
    use serde::Serialize;
    use utoipa::ToSchema;

    #[derive(ToSchema, Serialize)]
    struct Response {
        #[schema(inline)]
        eggs: Pagination<crate::models::nest_egg::AdminApiNestEgg>,
    }

    #[utoipa::path(get, path = "/", responses(
        (status = OK, body = inline(Response)),
    ), params(
        (
            "nest" = i32,
            description = "The nest ID",
            example = "1",
        ),
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
    ))]
    pub async fn route(
        state: GetState,
        nest: GetNest,
        Query(params): Query<PaginationParamsWithSearch>,
    ) -> (StatusCode, axum::Json<serde_json::Value>) {
        if let Err(errors) = crate::utils::validate_data(&params) {
            return (
                StatusCode::UNAUTHORIZED,
                axum::Json(ApiError::new_strings_value(errors)),
            );
        }

        let eggs = NestEgg::by_nest_id_with_pagination(
            &state.database,
            nest.id,
            params.page,
            params.per_page,
            params.search.as_deref(),
        )
        .await;

        (
            StatusCode::OK,
            axum::Json(
                serde_json::to_value(Response {
                    eggs: Pagination {
                        total: eggs.total,
                        per_page: eggs.per_page,
                        page: eggs.page,
                        data: eggs
                            .data
                            .into_iter()
                            .map(|egg| egg.into_admin_api_object())
                            .collect(),
                    },
                })
                .unwrap(),
            ),
        )
    }
}

mod post {
    use crate::{
        models::nest_egg::NestEgg,
        routes::{
            ApiError, GetState,
            api::{admin::nests::_nest_::GetNest, client::GetUserActivityLogger},
        },
    };
    use axum::http::StatusCode;
    use indexmap::IndexMap;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;
    use validator::Validate;

    #[derive(ToSchema, Validate, Deserialize)]
    pub struct Payload {
        #[validate(length(min = 2, max = 255))]
        #[schema(min_length = 2, max_length = 255)]
        author: String,
        #[validate(length(min = 3, max = 255))]
        #[schema(min_length = 3, max_length = 255)]
        name: String,
        #[validate(length(max = 1024))]
        #[schema(max_length = 1024)]
        description: Option<String>,

        #[schema(inline)]
        config_files: Vec<crate::models::nest_egg::ProcessConfigurationFile>,
        #[schema(inline)]
        config_startup: crate::models::nest_egg::NestEggConfigStartup,
        #[schema(inline)]
        config_stop: crate::models::nest_egg::NestEggConfigStop,
        #[schema(inline)]
        config_script: crate::models::nest_egg::NestEggConfigScript,
        #[schema(inline)]
        config_allocations: crate::models::nest_egg::NestEggConfigAllocations,

        #[validate(length(min = 1, max = 255))]
        #[schema(min_length = 1, max_length = 255)]
        startup: String,
        force_outgoing_ip: bool,

        features: Vec<String>,
        docker_images: IndexMap<String, String>,
        file_denylist: Vec<String>,
    }

    #[derive(ToSchema, Serialize)]
    struct Response {
        egg: crate::models::nest_egg::AdminApiNestEgg,
    }

    #[utoipa::path(post, path = "/", responses(
        (status = OK, body = inline(Response)),
        (status = BAD_REQUEST, body = ApiError),
        (status = CONFLICT, body = ApiError),
    ), params(
        (
            "nest" = i32,
            description = "The nest ID",
            example = "1",
        )
    ), request_body = inline(Payload))]
    pub async fn route(
        state: GetState,
        nest: GetNest,
        activity_logger: GetUserActivityLogger,
        axum::Json(data): axum::Json<Payload>,
    ) -> (StatusCode, axum::Json<serde_json::Value>) {
        if let Err(errors) = crate::utils::validate_data(&data) {
            return (
                StatusCode::BAD_REQUEST,
                axum::Json(ApiError::new_strings_value(errors)),
            );
        }

        let egg = match NestEgg::create(
            &state.database,
            nest.id,
            &data.author,
            &data.name,
            data.description.as_deref(),
            data.config_files,
            data.config_startup,
            data.config_stop,
            data.config_script,
            data.config_allocations,
            &data.startup,
            data.force_outgoing_ip,
            &data.features,
            data.docker_images,
            &data.file_denylist,
        )
        .await
        {
            Ok(egg) => egg,
            Err(err) if err.to_string().contains("unique constraint") => {
                return (
                    StatusCode::CONFLICT,
                    axum::Json(ApiError::new_value(&["egg with name already exists"])),
                );
            }
            Err(err) => {
                tracing::error!("failed to create egg: {:#?}", err);

                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(ApiError::new_value(&["failed to create egg"])),
                );
            }
        };

        activity_logger
            .log(
                "admin:egg.create",
                serde_json::json!({
                    "nest_id": nest.id,

                    "author": egg.author,
                    "name": egg.name,
                    "description": egg.description,

                    "config_files": egg.config_files,
                    "config_startup": egg.config_startup,
                    "config_stop": egg.config_stop,
                    "config_script": egg.config_script,
                    "config_allocations": egg.config_allocations,

                    "startup": egg.startup,
                    "force_outgoing_ip": egg.force_outgoing_ip,

                    "features": egg.features,
                    "docker_images": egg.docker_images,
                    "file_denylist": egg.file_denylist,
                }),
            )
            .await;

        (
            StatusCode::OK,
            axum::Json(
                serde_json::to_value(Response {
                    egg: egg.into_admin_api_object(),
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
        .nest("/import", import::router(state))
        .nest("/{egg}", _egg_::router(state))
        .with_state(state.clone())
}
