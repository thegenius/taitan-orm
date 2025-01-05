mod state;

use crate::state::AppState;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::routing::{delete, patch, post};
use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::Arc;
use tracing::info;
use taitan_orm::prelude::*;

#[derive(Schema, Clone, Debug, Serialize, Deserialize)]
#[table_name = "user"]
#[serde_struct = "selected"]
#[serde_struct = "mutation"]
#[serde_struct = "unique"]
pub struct User {
    #[primary_key]
    id: i32,
    name: String,
    age: Optional<i32>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let mut shared_state = Arc::new(
        AppState::build_sqlite("./workspace", "test.db")
            .await
            .unwrap(),
    );

    shared_state
        .execute_plain("DROP TABLE IF EXISTS `user`")
        .await
        .unwrap();
    shared_state.execute_plain(
        "CREATE TABLE IF NOT EXISTS `user`(`id` INT PRIMARY KEY, `age` INT, `name` VARCHAR(64))",
    ).await.unwrap();

    // prepare one user for select
    let entity = User {
        id: 1,
        name: "Allen".to_string(),
        age: Optional::Some(23),
    };
    let result = shared_state.insert(&entity).await.unwrap();
    assert_eq!(result, true);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/user", get(query_user_by_id))
        .route("/user", post(create_user))
        .route("/user", patch(update_user))
        .route("/user/{id}", delete(delete_user))
        .with_state(shared_state.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    info!("Server listening on 127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(entity): Json<User>,
) -> impl IntoResponse {
    let success = state.insert(&entity).await.unwrap();
    format!("insert {}", success)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateCommand {
    pub user_mutation: UserMutation,
    pub user_primary: UserPrimary,
}
async fn update_user(
    State(state): State<Arc<AppState>>,
    Json(command): Json<UpdateCommand>,
) -> impl IntoResponse {
    let success = state
        .update(&command.user_mutation, &command.user_primary)
        .await
        .unwrap();
    format!("update {}", success)
}

async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let primary = UserPrimary {id};
    let success = state
        .delete(&primary)
        .await
        .unwrap();
    format!("update {}", success)
}

async fn query_user_by_id(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match params.get("id") {
        Some(id) => {
            let id_index = id.parse::<i32>().unwrap();
            let selection = UserSelectedEntity::full_fields();
            let entity: Option<UserSelectedEntity> = state
                .select(&selection, &UserPrimary { id: id_index })
                .await
                .unwrap();
            let json = serde_json::to_string(&entity).unwrap();
            format!("Hello, {}!", json)
        }
        None => "Missing 'id' parameter".to_string(),
    }
}
