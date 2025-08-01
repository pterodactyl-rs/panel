// This file is auto-generated from OpenAPI spec. Do not edit manually.
use super::*;
use reqwest::{Client, Method, StatusCode};
use serde::de::DeserializeOwned;
use std::sync::LazyLock;

static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .user_agent("pterodactyl-rs panel")
        .build()
        .expect("Failed to create reqwest client")
});

type Algorithm = String;
type Game = String;

#[inline]
async fn request_impl<T: DeserializeOwned + 'static>(
    client: &WingsClient,
    method: Method,
    endpoint: impl AsRef<str>,
    body: Option<&impl Serialize>,
    body_raw: Option<String>,
) -> Result<T, (StatusCode, super::ApiError)> {
    let url = format!(
        "{}{}",
        client.base_url.trim_end_matches('/'),
        endpoint.as_ref()
    );
    let mut request = CLIENT.request(method, &url);

    if !client.token.is_empty() {
        request = request.header("Authorization", format!("Bearer {}", client.token));
    }

    if let Some(body) = body {
        request = request.json(body);
    } else if let Some(body_raw) = body_raw {
        request = request.body(body_raw);
    }

    match request.send().await {
        Ok(response) => {
            if response.status().is_success() {
                if std::any::type_name::<T>() == std::any::type_name::<String>() {
                    return match response.text().await {
                        Ok(text) => Ok(*(Box::new(text) as Box<dyn std::any::Any>)
                            .downcast::<T>()
                            .unwrap()),
                        Err(err) => Err((
                            StatusCode::PRECONDITION_FAILED,
                            super::ApiError {
                                error: err.to_string(),
                            },
                        )),
                    };
                }

                match response.json().await {
                    Ok(data) => Ok(data),
                    Err(err) => Err((
                        StatusCode::PRECONDITION_FAILED,
                        super::ApiError {
                            error: err.to_string(),
                        },
                    )),
                }
            } else {
                Err((
                    response.status(),
                    response.json().await.unwrap_or_else(|err| super::ApiError {
                        error: err.to_string(),
                    }),
                ))
            }
        }
        Err(err) => Err((
            StatusCode::PRECONDITION_FAILED,
            super::ApiError {
                error: err.to_string(),
            },
        )),
    }
}

pub struct WingsClient {
    base_url: String,
    token: String,
}

impl WingsClient {
    #[inline]
    pub fn new(base_url: String, token: String) -> Self {
        Self { base_url, token }
    }

    pub async fn post_backups_sync(
        &self,
    ) -> Result<super::backups_sync::post::Response200, (StatusCode, super::ApiError)> {
        request_impl(
            self,
            Method::POST,
            "/api/backups/sync",
            None::<&usize>,
            None,
        )
        .await
    }

    pub async fn delete_backups_backup(
        &self,
        backup: uuid::Uuid,
    ) -> Result<super::backups_backup::delete::Response202, (StatusCode, super::ApiError)> {
        request_impl(
            self,
            Method::DELETE,
            format!("/api/backups/{backup}"),
            None::<&usize>,
            None,
        )
        .await
    }

    pub async fn get_extensions(
        &self,
    ) -> Result<super::extensions::get::Response200, (StatusCode, super::ApiError)> {
        request_impl(self, Method::GET, "/api/extensions", None::<&usize>, None).await
    }

    pub async fn get_servers(
        &self,
    ) -> Result<super::servers::get::Response200, (StatusCode, super::ApiError)> {
        request_impl(self, Method::GET, "/api/servers", None::<&usize>, None).await
    }

    pub async fn post_servers(
        &self,
        data: &super::servers::post::RequestBody,
    ) -> Result<super::servers::post::Response200, (StatusCode, super::ApiError)> {
        request_impl(self, Method::POST, "/api/servers", Some(data), None).await
    }

    pub async fn get_servers_server(
        &self,
        server: uuid::Uuid,
    ) -> Result<super::servers_server::get::Response200, (StatusCode, super::ApiError)> {
        request_impl(
            self,
            Method::GET,
            format!("/api/servers/{server}"),
            None::<&usize>,
            None,
        )
        .await
    }

    pub async fn delete_servers_server(
        &self,
        server: uuid::Uuid,
    ) -> Result<super::servers_server::delete::Response200, (StatusCode, super::ApiError)> {
        request_impl(
            self,
            Method::DELETE,
            format!("/api/servers/{server}"),
            None::<&usize>,
            None,
        )
        .await
    }

    pub async fn post_servers_server_backup(
        &self,
        server: uuid::Uuid,
        data: &super::servers_server_backup::post::RequestBody,
    ) -> Result<super::servers_server_backup::post::Response202, (StatusCode, super::ApiError)>
    {
        request_impl(
            self,
            Method::POST,
            format!("/api/servers/{server}/backup"),
            Some(data),
            None,
        )
        .await
    }

    pub async fn delete_servers_server_backup_backup(
        &self,
        server: uuid::Uuid,
        backup: uuid::Uuid,
    ) -> Result<
        super::servers_server_backup_backup::delete::Response202,
        (StatusCode, super::ApiError),
    > {
        request_impl(
            self,
            Method::DELETE,
            format!("/api/servers/{server}/backup/{backup}"),
            None::<&usize>,
            None,
        )
        .await
    }

    pub async fn post_servers_server_backup_backup_restore(
        &self,
        server: uuid::Uuid,
        backup: uuid::Uuid,
        data: &super::servers_server_backup_backup_restore::post::RequestBody,
    ) -> Result<
        super::servers_server_backup_backup_restore::post::Response202,
        (StatusCode, super::ApiError),
    > {
        request_impl(
            self,
            Method::POST,
            format!("/api/servers/{server}/backup/{backup}/restore"),
            Some(data),
            None,
        )
        .await
    }

    pub async fn post_servers_server_commands(
        &self,
        server: uuid::Uuid,
        data: &super::servers_server_commands::post::RequestBody,
    ) -> Result<super::servers_server_commands::post::Response200, (StatusCode, super::ApiError)>
    {
        request_impl(
            self,
            Method::POST,
            format!("/api/servers/{server}/commands"),
            Some(data),
            None,
        )
        .await
    }

    pub async fn post_servers_server_files_chmod(
        &self,
        server: uuid::Uuid,
        data: &super::servers_server_files_chmod::post::RequestBody,
    ) -> Result<super::servers_server_files_chmod::post::Response200, (StatusCode, super::ApiError)>
    {
        request_impl(
            self,
            Method::POST,
            format!("/api/servers/{server}/files/chmod"),
            Some(data),
            None,
        )
        .await
    }

    pub async fn post_servers_server_files_compress(
        &self,
        server: uuid::Uuid,
        data: &super::servers_server_files_compress::post::RequestBody,
    ) -> Result<
        super::servers_server_files_compress::post::Response200,
        (StatusCode, super::ApiError),
    > {
        request_impl(
            self,
            Method::POST,
            format!("/api/servers/{server}/files/compress"),
            Some(data),
            None,
        )
        .await
    }

    pub async fn get_servers_server_files_contents(
        &self,
        server: uuid::Uuid,
        file: &str,
        download: bool,
        max_size: u64,
    ) -> Result<super::servers_server_files_contents::get::Response200, (StatusCode, super::ApiError)>
    {
        let file = urlencoding::encode(file);
        request_impl(self, Method::GET, format!("/api/servers/{server}/files/contents?file={file}&download={download}&max_size={max_size}"), None::<&usize>, None).await
    }

    pub async fn post_servers_server_files_copy(
        &self,
        server: uuid::Uuid,
        data: &super::servers_server_files_copy::post::RequestBody,
    ) -> Result<super::servers_server_files_copy::post::Response200, (StatusCode, super::ApiError)>
    {
        request_impl(
            self,
            Method::POST,
            format!("/api/servers/{server}/files/copy"),
            Some(data),
            None,
        )
        .await
    }

    pub async fn post_servers_server_files_create_directory(
        &self,
        server: uuid::Uuid,
        data: &super::servers_server_files_create_directory::post::RequestBody,
    ) -> Result<
        super::servers_server_files_create_directory::post::Response200,
        (StatusCode, super::ApiError),
    > {
        request_impl(
            self,
            Method::POST,
            format!("/api/servers/{server}/files/create-directory"),
            Some(data),
            None,
        )
        .await
    }

    pub async fn post_servers_server_files_decompress(
        &self,
        server: uuid::Uuid,
        data: &super::servers_server_files_decompress::post::RequestBody,
    ) -> Result<
        super::servers_server_files_decompress::post::Response200,
        (StatusCode, super::ApiError),
    > {
        request_impl(
            self,
            Method::POST,
            format!("/api/servers/{server}/files/decompress"),
            Some(data),
            None,
        )
        .await
    }

    pub async fn post_servers_server_files_delete(
        &self,
        server: uuid::Uuid,
        data: &super::servers_server_files_delete::post::RequestBody,
    ) -> Result<super::servers_server_files_delete::post::Response200, (StatusCode, super::ApiError)>
    {
        request_impl(
            self,
            Method::POST,
            format!("/api/servers/{server}/files/delete"),
            Some(data),
            None,
        )
        .await
    }

    pub async fn get_servers_server_files_fingerprints(
        &self,
        server: uuid::Uuid,
        algorithm: Algorithm,
        files: Vec<String>,
    ) -> Result<
        super::servers_server_files_fingerprints::get::Response200,
        (StatusCode, super::ApiError),
    > {
        let files = files
            .into_iter()
            .map(|s| urlencoding::encode(&s).into_owned())
            .collect::<Vec<_>>()
            .join("&files=");
        request_impl(
            self,
            Method::GET,
            format!("/api/servers/{server}/files/fingerprints?algorithm={algorithm}&files={files}"),
            None::<&usize>,
            None,
        )
        .await
    }

    pub async fn get_servers_server_files_list(
        &self,
        server: uuid::Uuid,
        directory: &str,
        ignored: Vec<String>,
        per_page: u64,
        page: u64,
    ) -> Result<super::servers_server_files_list::get::Response200, (StatusCode, super::ApiError)>
    {
        let directory = urlencoding::encode(directory);
        let ignored = ignored
            .into_iter()
            .map(|s| urlencoding::encode(&s).into_owned())
            .collect::<Vec<_>>()
            .join("&ignored=");
        request_impl(self, Method::GET, format!("/api/servers/{server}/files/list?directory={directory}&ignored={ignored}&per_page={per_page}&page={page}"), None::<&usize>, None).await
    }

    pub async fn get_servers_server_files_list_directory(
        &self,
        server: uuid::Uuid,
        directory: &str,
    ) -> Result<
        super::servers_server_files_list_directory::get::Response200,
        (StatusCode, super::ApiError),
    > {
        let directory = urlencoding::encode(directory);
        request_impl(
            self,
            Method::GET,
            format!("/api/servers/{server}/files/list-directory?directory={directory}"),
            None::<&usize>,
            None,
        )
        .await
    }

    pub async fn get_servers_server_files_pull(
        &self,
        server: uuid::Uuid,
    ) -> Result<super::servers_server_files_pull::get::Response200, (StatusCode, super::ApiError)>
    {
        request_impl(
            self,
            Method::GET,
            format!("/api/servers/{server}/files/pull"),
            None::<&usize>,
            None,
        )
        .await
    }

    pub async fn post_servers_server_files_pull(
        &self,
        server: uuid::Uuid,
        data: &super::servers_server_files_pull::post::RequestBody,
    ) -> Result<super::servers_server_files_pull::post::Response200, (StatusCode, super::ApiError)>
    {
        request_impl(
            self,
            Method::POST,
            format!("/api/servers/{server}/files/pull"),
            Some(data),
            None,
        )
        .await
    }

    pub async fn delete_servers_server_files_pull_pull(
        &self,
        server: uuid::Uuid,
        pull: uuid::Uuid,
    ) -> Result<
        super::servers_server_files_pull_pull::delete::Response200,
        (StatusCode, super::ApiError),
    > {
        request_impl(
            self,
            Method::DELETE,
            format!("/api/servers/{server}/files/pull/{pull}"),
            None::<&usize>,
            None,
        )
        .await
    }

    pub async fn put_servers_server_files_rename(
        &self,
        server: uuid::Uuid,
        data: &super::servers_server_files_rename::put::RequestBody,
    ) -> Result<super::servers_server_files_rename::put::Response200, (StatusCode, super::ApiError)>
    {
        request_impl(
            self,
            Method::PUT,
            format!("/api/servers/{server}/files/rename"),
            Some(data),
            None,
        )
        .await
    }

    pub async fn post_servers_server_files_search(
        &self,
        server: uuid::Uuid,
        data: &super::servers_server_files_search::post::RequestBody,
    ) -> Result<super::servers_server_files_search::post::Response200, (StatusCode, super::ApiError)>
    {
        request_impl(
            self,
            Method::POST,
            format!("/api/servers/{server}/files/search"),
            Some(data),
            None,
        )
        .await
    }

    pub async fn post_servers_server_files_write(
        &self,
        server: uuid::Uuid,
        file: &str,
        data: super::servers_server_files_write::post::RequestBody,
    ) -> Result<super::servers_server_files_write::post::Response200, (StatusCode, super::ApiError)>
    {
        let file = urlencoding::encode(file);
        request_impl(
            self,
            Method::POST,
            format!("/api/servers/{server}/files/write?file={file}"),
            None::<&usize>,
            Some(data),
        )
        .await
    }

    pub async fn get_servers_server_logs(
        &self,
        server: uuid::Uuid,
    ) -> Result<super::servers_server_logs::get::Response200, (StatusCode, super::ApiError)> {
        request_impl(
            self,
            Method::GET,
            format!("/api/servers/{server}/logs"),
            None::<&usize>,
            None,
        )
        .await
    }

    pub async fn post_servers_server_power(
        &self,
        server: uuid::Uuid,
        data: &super::servers_server_power::post::RequestBody,
    ) -> Result<super::servers_server_power::post::Response202, (StatusCode, super::ApiError)> {
        request_impl(
            self,
            Method::POST,
            format!("/api/servers/{server}/power"),
            Some(data),
            None,
        )
        .await
    }

    pub async fn post_servers_server_reinstall(
        &self,
        server: uuid::Uuid,
        data: &super::servers_server_reinstall::post::RequestBody,
    ) -> Result<super::servers_server_reinstall::post::Response202, (StatusCode, super::ApiError)>
    {
        request_impl(
            self,
            Method::POST,
            format!("/api/servers/{server}/reinstall"),
            Some(data),
            None,
        )
        .await
    }

    pub async fn post_servers_server_script(
        &self,
        server: uuid::Uuid,
        data: &super::servers_server_script::post::RequestBody,
    ) -> Result<super::servers_server_script::post::Response200, (StatusCode, super::ApiError)>
    {
        request_impl(
            self,
            Method::POST,
            format!("/api/servers/{server}/script"),
            Some(data),
            None,
        )
        .await
    }

    pub async fn post_servers_server_sync(
        &self,
        server: uuid::Uuid,
    ) -> Result<super::servers_server_sync::post::Response200, (StatusCode, super::ApiError)> {
        request_impl(
            self,
            Method::POST,
            format!("/api/servers/{server}/sync"),
            None::<&usize>,
            None,
        )
        .await
    }

    pub async fn delete_servers_server_transfer(
        &self,
        server: uuid::Uuid,
    ) -> Result<super::servers_server_transfer::delete::Response200, (StatusCode, super::ApiError)>
    {
        request_impl(
            self,
            Method::DELETE,
            format!("/api/servers/{server}/transfer"),
            None::<&usize>,
            None,
        )
        .await
    }

    pub async fn post_servers_server_transfer(
        &self,
        server: uuid::Uuid,
        data: &super::servers_server_transfer::post::RequestBody,
    ) -> Result<super::servers_server_transfer::post::Response202, (StatusCode, super::ApiError)>
    {
        request_impl(
            self,
            Method::POST,
            format!("/api/servers/{server}/transfer"),
            Some(data),
            None,
        )
        .await
    }

    pub async fn get_servers_server_version(
        &self,
        server: uuid::Uuid,
        game: Game,
    ) -> Result<super::servers_server_version::get::Response200, (StatusCode, super::ApiError)>
    {
        request_impl(
            self,
            Method::GET,
            format!("/api/servers/{server}/version?game={game}"),
            None::<&usize>,
            None,
        )
        .await
    }

    pub async fn post_servers_server_ws_deny(
        &self,
        server: uuid::Uuid,
        data: &super::servers_server_ws_deny::post::RequestBody,
    ) -> Result<super::servers_server_ws_deny::post::Response200, (StatusCode, super::ApiError)>
    {
        request_impl(
            self,
            Method::POST,
            format!("/api/servers/{server}/ws/deny"),
            Some(data),
            None,
        )
        .await
    }

    pub async fn post_servers_server_ws_permissions(
        &self,
        server: uuid::Uuid,
        data: &super::servers_server_ws_permissions::post::RequestBody,
    ) -> Result<
        super::servers_server_ws_permissions::post::Response200,
        (StatusCode, super::ApiError),
    > {
        request_impl(
            self,
            Method::POST,
            format!("/api/servers/{server}/ws/permissions"),
            Some(data),
            None,
        )
        .await
    }

    pub async fn get_stats(
        &self,
    ) -> Result<super::stats::get::Response200, (StatusCode, super::ApiError)> {
        request_impl(self, Method::GET, "/api/stats", None::<&usize>, None).await
    }

    pub async fn get_system(
        &self,
    ) -> Result<super::system::get::Response200, (StatusCode, super::ApiError)> {
        request_impl(self, Method::GET, "/api/system", None::<&usize>, None).await
    }

    pub async fn post_transfers(
        &self,
    ) -> Result<super::transfers::post::Response200, (StatusCode, super::ApiError)> {
        request_impl(self, Method::POST, "/api/transfers", None::<&usize>, None).await
    }

    pub async fn delete_transfers_server(
        &self,
        server: uuid::Uuid,
    ) -> Result<super::transfers_server::delete::Response200, (StatusCode, super::ApiError)> {
        request_impl(
            self,
            Method::DELETE,
            format!("/api/transfers/{server}"),
            None::<&usize>,
            None,
        )
        .await
    }

    pub async fn post_update(
        &self,
        data: &super::update::post::RequestBody,
    ) -> Result<super::update::post::Response200, (StatusCode, super::ApiError)> {
        request_impl(self, Method::POST, "/api/update", Some(data), None).await
    }
}
