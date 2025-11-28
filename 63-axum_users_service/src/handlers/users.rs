use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;

use crate::models::user::{CreateUser, UpdateUser, User};
use crate::state::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

/// POST /users
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (name, email, age)
        VALUES ($1, $2, $3)
        RETURNING id, name, email, age, created_at
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(payload.age)
    .fetch_one(&state.pool)
    .await
    .map_err(internal_error)?;

    Ok((StatusCode::CREATED, Json(user)))
}

/// GET /users
pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name, email, age, created_at
        FROM users
        ORDER BY id
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(internal_error)?;

    Ok(Json(users))
}

/// GET /users/:id
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<User>, (StatusCode, String)> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name, email, age, created_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    .map_err(internal_error)?;

    match user {
        Some(u) => Ok(Json(u)),
        None => Err((StatusCode::NOT_FOUND, format!("User {} not found", id))),
    }
}

/// PUT /users/:id
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let existing = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name, email, age, created_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    .map_err(internal_error)?;

    let existing = match existing {
        Some(u) => u,
        None => return Err((StatusCode::NOT_FOUND, format!("User {} not found", id))),
    };

    let new_name = payload.name.unwrap_or(existing.name);
    let new_email = payload.email.unwrap_or(existing.email);
    let new_age = payload.age.or(existing.age);

    let updated = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET name = $1,
            email = $2,
            age = $3
        WHERE id = $4
        RETURNING id, name, email, age, created_at
        "#,
    )
    .bind(new_name)
    .bind(new_email)
    .bind(new_age)
    .bind(id)
    .fetch_one(&state.pool)
    .await
    .map_err(internal_error)?;

    Ok(Json(updated))
}

/// DELETE /users/:id
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(internal_error)?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, format!("User {} not found", id)));
    }

    Ok(StatusCode::NO_CONTENT)
}

fn internal_error<E: std::fmt::Display>(err: E) -> (StatusCode, String) {
    tracing::error!("internal error: {}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".into())
}
