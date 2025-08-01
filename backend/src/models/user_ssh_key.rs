use super::BaseModel;
use serde::{Deserialize, Serialize};
use sqlx::{Row, postgres::PgRow};
use std::collections::BTreeMap;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize)]
pub struct UserSshKey {
    pub id: i32,

    pub name: String,
    pub fingerprint: String,

    pub created: chrono::NaiveDateTime,
}

impl BaseModel for UserSshKey {
    #[inline]
    fn columns(prefix: Option<&str>, table: Option<&str>) -> BTreeMap<String, String> {
        let prefix = prefix.unwrap_or_default();
        let table = table.unwrap_or("user_ssh_keys");

        BTreeMap::from([
            (format!("{table}.id"), format!("{prefix}id")),
            (format!("{table}.name"), format!("{prefix}name")),
            (
                format!("{table}.fingerprint"),
                format!("{prefix}fingerprint"),
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
            fingerprint: row.get(format!("{prefix}fingerprint").as_str()),
            created: row.get(format!("{prefix}created").as_str()),
        }
    }
}

impl UserSshKey {
    #[inline]
    pub async fn create(
        database: &crate::database::Database,
        user_id: i32,
        name: &str,
        public_key: russh::keys::PublicKey,
    ) -> Result<Self, sqlx::Error> {
        let row = sqlx::query(&format!(
            r#"
            INSERT INTO user_ssh_keys (user_id, name, fingerprint, public_key, created)
            VALUES ($1, $2, $3, $4, NOW())
            RETURNING {}
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(user_id)
        .bind(name)
        .bind(
            public_key
                .fingerprint(russh::keys::HashAlg::Sha256)
                .to_string(),
        )
        .bind(public_key.to_bytes().unwrap())
        .fetch_one(database.write())
        .await?;

        Ok(Self::map(None, &row))
    }

    pub async fn by_fingerprint(
        database: &crate::database::Database,
        user_id: i32,
        fingerprint: String,
    ) -> Result<Option<Self>, sqlx::Error> {
        let row = sqlx::query(&format!(
            r#"
            SELECT {}
            FROM user_ssh_keys
            WHERE user_ssh_keys.user_id = $1 AND user_ssh_keys.fingerprint = $2
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(user_id)
        .bind(if fingerprint.starts_with("SHA256:") {
            fingerprint
        } else {
            format!("SHA256:{fingerprint}")
        })
        .fetch_optional(database.read())
        .await?;

        Ok(row.map(|row| Self::map(None, &row)))
    }

    pub async fn by_user_id_with_pagination(
        database: &crate::database::Database,
        user_id: i32,
        page: i64,
        per_page: i64,
        search: Option<&str>,
    ) -> Result<super::Pagination<Self>, sqlx::Error> {
        let offset = (page - 1) * per_page;

        let rows = sqlx::query(&format!(
            r#"
            SELECT {}, COUNT(*) OVER() AS total_count
            FROM user_ssh_keys
            WHERE user_ssh_keys.user_id = $1 AND ($2 IS NULL OR user_ssh_keys.name ILIKE '%' || $2 || '%')
            ORDER BY user_ssh_keys.id ASC
            LIMIT $3 OFFSET $4
            "#,
            Self::columns_sql(None, None)
        ))
        .bind(user_id)
        .bind(search)
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

    pub async fn delete_by_id(
        database: &crate::database::Database,
        id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM user_ssh_keys
            WHERE user_ssh_keys.id = $1
            "#,
        )
        .bind(id)
        .execute(database.write())
        .await?;

        Ok(())
    }

    #[inline]
    pub fn into_api_object(self) -> ApiUserSshKey {
        ApiUserSshKey {
            id: self.id,
            name: self.name,
            fingerprint: self.fingerprint,
            created: self.created.and_utc(),
        }
    }
}

#[derive(ToSchema, Serialize)]
#[schema(title = "UserSshKey")]
pub struct ApiUserSshKey {
    pub id: i32,

    pub name: String,
    pub fingerprint: String,

    pub created: chrono::DateTime<chrono::Utc>,
}
