use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::handlers::users::{
    create_user, delete_user, get_user, health_check, list_users, update_user,
};
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/users", post(create_user).get(list_users))
        .route(
            "/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        )
        .with_state(state)
}
