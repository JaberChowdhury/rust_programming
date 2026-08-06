use crate::models::user::User;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_user(
    pool: &PgPool,
    username: &str,
    password_hash: &str,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, password_hash)
        VALUES ($1, $2)
        RETURNING id, username, password_hash, created_at, updated_at
        "#,
    )
    .bind(username)
    .bind(password_hash)
    .fetch_one(pool)
    .await
}

pub async fn get_user_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        SELECT id, username, password_hash, created_at, updated_at
        FROM users
        WHERE username = $1
        "#,
    )
    .bind(username)
    .fetch_optional(pool)
    .await
}

pub async fn get_user(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        SELECT id, username, password_hash, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn list_users(pool: &PgPool, page: i64, per_page: i64) -> Result<Vec<User>, sqlx::Error> {
    let offset = (page - 1) * per_page;
    sqlx::query_as::<_, User>(
        r#"
        SELECT id, username, password_hash, created_at, updated_at
        FROM users
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn update_user(
    pool: &PgPool,
    id: Uuid,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET username = $1, updated_at = NOW()
        WHERE id = $2
        RETURNING id, username, password_hash, created_at, updated_at
        "#,
    )
    .bind(username)
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn delete_user(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}
