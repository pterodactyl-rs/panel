use super::BaseModel;
use serde::{Deserialize, Serialize};
use sqlx::{Row, postgres::PgRow};
use std::collections::BTreeMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize)]
pub struct Mount {
    pub id: i32,

    pub name: String,
    pub description: Option<String>,

    pub source: String,
    pub target: String,

    pub read_only: bool,
    pub user_mountable: bool,

    pub created: chrono::NaiveDateTime,
}

impl BaseModel for Mount {
    #[inline]
    fn columns(prefix: Option<&str>, table: Option<&str>) -> BTreeMap<String, String> {
        let prefix = prefix.unwrap_or_default();
        let table = table.unwrap_or("mounts");

        BTreeMap::from([
            (format!("{table}.id"), format!("{prefix}id")),
            (format!("{table}.name"), format!("{prefix}name")),
            (
                format!("{table}.description"),
                format!("{prefix}description"),
            ),
            (format!("{table}.source"), format!("{prefix}source")),
            (format!("{table}.target"), format!("{prefix}target")),
            (format!("{table}.read_only"), format!("{prefix}read_only")),
            (
                format!("{table}.user_mountable"),
                format!("{prefix}user_mountable"),
            ),
            (format!("{table}.created"), format!("{prefix}created")),
        ])
    }

    #[inline]
    fn map(prefix: Option<&str>, row: &PgRow) -> Self {
        let prefix = prefix.unwrap_or_default();

        Self {
            id: row.get(format!("{prefix}id").as_str()),
            name: row.get(format!("{prefix}name").as_str()),
            description: row.get(format!("{prefix}description").as_str()),
            source: row.get(format!("{prefix}source").as_str()),
            target: row.get(format!("{prefix}target").as_str()),
            read_only: row.get(format!("{prefix}read_only").as_str()),
            user_mountable: row.get(format!("{prefix}user_mountable").as_str()),
            created: row.get(format!("{prefix}created").as_str()),
        }
    }
}

impl Mount {
    pub async fn new(
        database: &crate::database::Database,
        name: &str,
        description: Option<&str>,
        source: &str,
        target: &str,
        read_only: bool,
        user_mountable: bool,
    ) -> Self {
        let row = sqlx::query(&format!(
            r#"
            INSERT INTO mounts (name, description, source, target, read_only, user_mountable)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING {}
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(name)
        .bind(description)
        .bind(source)
        .bind(target)
        .bind(read_only)
        .bind(user_mountable)
        .fetch_one(database.write())
        .await
        .unwrap();

        Self::map(None, &row)
    }

    pub async fn by_id(database: &crate::database::Database, id: i32) -> Option<Self> {
        let row = sqlx::query(&format!(
            r#"
            SELECT {}
            FROM mounts
            WHERE mounts.id = $1
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(id)
        .fetch_optional(database.read())
        .await
        .unwrap();

        row.map(|row| Self::map(None, &row))
    }

    pub async fn delete_by_id(database: &crate::database::Database, id: i32) {
        sqlx::query(
            r#"
            DELETE FROM mounts
            WHERE mounts.id = $1
            "#,
        )
        .bind(id)
        .execute(database.write())
        .await
        .unwrap();
    }

    #[inline]
    pub fn into_admin_api_object(self) -> AdminApiMount {
        AdminApiMount {
            id: self.id,
            name: self.name,
            description: self.description,
            source: self.source,
            target: self.target,
            read_only: self.read_only,
            user_mountable: self.user_mountable,
            created: self.created.and_utc(),
        }
    }
}

#[derive(ToSchema, Serialize)]
#[schema(title = "Mount")]
pub struct AdminApiMount {
    pub id: i32,

    pub name: String,
    pub description: Option<String>,

    pub source: String,
    pub target: String,

    pub read_only: bool,
    pub user_mountable: bool,

    pub created: chrono::DateTime<chrono::Utc>,
}
