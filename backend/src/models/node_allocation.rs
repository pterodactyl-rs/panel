use super::BaseModel;
use serde::{Deserialize, Serialize};
use sqlx::{Row, postgres::PgRow};
use std::collections::BTreeMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone)]
pub struct NodeAllocation {
    pub id: i32,

    pub ip: sqlx::types::ipnetwork::IpNetwork,
    pub ip_alias: Option<String>,
    pub port: i32,

    pub created: chrono::NaiveDateTime,
}

impl BaseModel for NodeAllocation {
    #[inline]
    fn columns(prefix: Option<&str>, table: Option<&str>) -> BTreeMap<String, String> {
        let prefix = prefix.unwrap_or_default();
        let table = table.unwrap_or("node_allocations");

        BTreeMap::from([
            (format!("{table}.id"), format!("{prefix}id")),
            (format!("{table}.ip"), format!("{prefix}ip")),
            (format!("{table}.ip_alias"), format!("{prefix}ip_alias")),
            (format!("{table}.port"), format!("{prefix}port")),
            (format!("{table}.created"), format!("{prefix}created")),
        ])
    }

    #[inline]
    fn map(prefix: Option<&str>, row: &PgRow) -> Self {
        let prefix = prefix.unwrap_or_default();

        Self {
            id: row.get(format!("{prefix}id").as_str()),
            ip: row.get(format!("{prefix}ip").as_str()),
            ip_alias: row.get(format!("{prefix}ip_alias").as_str()),
            port: row.get(format!("{prefix}port").as_str()),
            created: row.get(format!("{prefix}created").as_str()),
        }
    }
}

impl NodeAllocation {
    pub async fn create(
        database: &crate::database::Database,
        node_id: i32,
        ip: &sqlx::types::ipnetwork::IpNetwork,
        ip_alias: Option<&str>,
        port: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO node_allocations (node_id, ip, ip_alias, port)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(node_id)
        .bind(ip)
        .bind(ip_alias)
        .bind(port)
        .execute(database.write())
        .await?;

        Ok(())
    }

    pub async fn by_node_id_id(
        database: &crate::database::Database,
        node_id: i32,
        id: i32,
    ) -> Result<Option<Self>, sqlx::Error> {
        let row = sqlx::query(&format!(
            r#"
            SELECT {}
            FROM node_allocations
            WHERE node_allocations.node_id = $1 AND node_allocations.id = $2
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(node_id)
        .bind(id)
        .fetch_optional(database.read())
        .await?;

        Ok(row.map(|row| Self::map(None, &row)))
    }

    pub async fn by_node_id_with_pagination(
        database: &crate::database::Database,
        node_id: i32,
        page: i64,
        per_page: i64,
    ) -> Result<super::Pagination<Self>, sqlx::Error> {
        let offset = (page - 1) * per_page;

        let rows = sqlx::query(&format!(
            r#"
            SELECT {}, COUNT(*) OVER() AS total_count
            FROM node_allocations
            WHERE node_allocations.node_id = $1
            ORDER BY node_allocations.ip, node_allocations.port
            LIMIT $2 OFFSET $3
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(node_id)
        .bind(per_page)
        .bind(offset)
        .fetch_all(database.read())
        .await?;

        Ok(super::Pagination {
            total: rows.first().map_or(0, |row| row.get("total_count")),
            per_page,
            page,
            data: rows.into_iter().map(|row| Self::map(None, &row)).collect(),
        })
    }

    pub async fn delete_by_ids(
        database: &crate::database::Database,
        ids: &[i32],
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM node_allocations
            WHERE node_allocations.id = ANY($1)
            "#,
        )
        .bind(ids)
        .execute(database.write())
        .await?;

        Ok(())
    }

    #[inline]
    pub fn into_admin_api_object(self) -> AdminApiNodeAllocation {
        AdminApiNodeAllocation {
            id: self.id,
            ip: self.ip.ip().to_string(),
            ip_alias: self.ip_alias,
            port: self.port,
            created: self.created.and_utc(),
        }
    }
}

#[derive(ToSchema, Serialize)]
#[schema(title = "NodeAllocation")]
pub struct AdminApiNodeAllocation {
    pub id: i32,

    pub ip: String,
    pub ip_alias: Option<String>,
    pub port: i32,

    pub created: chrono::DateTime<chrono::Utc>,
}
