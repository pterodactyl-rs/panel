use super::BaseModel;
use serde::{Deserialize, Serialize};
use sqlx::{Row, postgres::PgRow};
use std::collections::BTreeMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerAllocation {
    pub id: i32,
    pub allocation: super::node_allocation::NodeAllocation,

    pub notes: Option<String>,

    pub created: chrono::NaiveDateTime,
}

impl BaseModel for ServerAllocation {
    #[inline]
    fn columns(prefix: Option<&str>, table: Option<&str>) -> BTreeMap<String, String> {
        let prefix = prefix.unwrap_or_default();
        let table = table.unwrap_or("server_allocations");

        let mut columns = BTreeMap::from([
            (format!("{table}.id"), format!("{prefix}id")),
            (
                format!("{table}.allocation_id"),
                format!("{prefix}allocation_id"),
            ),
            (format!("{table}.notes"), format!("{prefix}notes")),
            (format!("{table}.created"), format!("{prefix}created")),
        ]);

        columns.extend(super::node_allocation::NodeAllocation::columns(
            Some("allocation_"),
            None,
        ));

        columns
    }

    #[inline]
    fn map(prefix: Option<&str>, row: &PgRow) -> Self {
        let prefix = prefix.unwrap_or_default();

        Self {
            id: row.get(format!("{prefix}id").as_str()),
            allocation: super::node_allocation::NodeAllocation::map(Some("allocation_"), row),
            notes: row.get(format!("{prefix}notes").as_str()),
            created: row.get(format!("{prefix}created").as_str()),
        }
    }
}

impl ServerAllocation {
    pub async fn new(
        database: &crate::database::Database,
        server_id: i32,
        allocation_id: i32,
    ) -> bool {
        sqlx::query(
            r#"
            INSERT INTO server_allocations (server_id, allocation_id)
            VALUES ($1, $2)
            "#,
        )
        .bind(server_id)
        .bind(allocation_id)
        .fetch_one(database.write())
        .await
        .is_ok()
    }

    pub async fn all_by_server_id(
        database: &crate::database::Database,
        server_id: i32,
    ) -> Vec<Self> {
        let rows = sqlx::query(&format!(
            r#"
            SELECT {}, {}
            FROM server_allocations
            JOIN mounts ON mounts.id = server_allocations.allocation_id
            WHERE server_allocations.server_id = $1
            "#,
            Self::columns_sql(None, None),
            super::node_allocation::NodeAllocation::columns_sql(Some("allocation_"), None)
        ))
        .bind(server_id)
        .fetch_all(database.read())
        .await
        .unwrap();

        rows.into_iter().map(|row| Self::map(None, &row)).collect()
    }

    pub async fn delete_by_id(
        database: &crate::database::Database,
        server_id: i32,
        allocation_id: i32,
    ) {
        sqlx::query(
            r#"
            DELETE FROM server_allocations
            WHERE server_id = $1 AND allocation_id = $2
            "#,
        )
        .bind(server_id)
        .bind(allocation_id)
        .execute(database.write())
        .await
        .unwrap();
    }

    #[inline]
    pub fn into_api_object(self, default: Option<i32>) -> ApiServerAllocation {
        ApiServerAllocation {
            ip: self.allocation.ip.ip().to_string(),
            ip_alias: self.allocation.ip_alias,
            port: self.allocation.port,
            notes: self.notes,
            is_default: default.is_some_and(|d| d == self.id),
            created: self.created.and_utc(),
        }
    }
}

#[derive(ToSchema, Serialize)]
#[schema(title = "ServerAllocation")]
pub struct ApiServerAllocation {
    pub ip: String,
    pub ip_alias: Option<String>,
    pub port: i32,

    pub notes: Option<String>,
    pub is_default: bool,

    pub created: chrono::DateTime<chrono::Utc>,
}
