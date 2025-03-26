mod state;

use crate::state::AppState;
use axum::debug_handler;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::routing::{delete, patch, post};
use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use taitan_orm::database::sqlite::prelude::*;
use taitan_orm::prelude::*;
use tracing::info;


#[derive(Schema, Clone, Debug, Serialize, Deserialize)]
#[table = "user"]
#[serde_struct = "selected"]
#[serde_struct = "mutation"]
#[serde_struct = "unique"]
#[primary(id)]
pub struct User {
    id: i32,
    name: String,
    age: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Users {
    user_a: User,
    user_b: User,
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
        age: Some(23),
    };
    shared_state.insert(&entity).await.unwrap();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/user", get(query_user_by_id))
        .route("/users_by_name", get(query_user_by_name_and_age))
        .route("/user", post(create_user))
        .route("/user", patch(update_user))
        .route("/user/{id}", delete(delete_user))
        .route("/users", post(create_users))
        .with_state(shared_state.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    info!("Server listening on 127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

// #[debug_handler]
async fn create_user(State(state): State<Arc<AppState>>, Json(entity): Json<User>) -> String {
    state.insert(&entity).await.unwrap();
    format!("insert success")
}

// #[debug_handler]
async fn create_users(State(state): State<Arc<AppState>>, Json(entity): Json<Users>) -> String {
    let mut trx = state.transaction().await.unwrap();
    trx.insert(&entity.user_a).await.unwrap();
    trx.insert(&entity.user_b).await.unwrap();
    trx.commit().await.unwrap();
    "insert all users success".to_string()
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
    let db: &SqliteDatabase = &*state;
    let success = state
        .update(&command.user_mutation, &command.user_primary)
        .await
        .unwrap();
    format!("update {}", success)
}

async fn delete_user(State(state): State<Arc<AppState>>, Path(id): Path<i32>) -> impl IntoResponse {
    let primary = UserPrimary { id };
    let success = state.delete(&primary).await.unwrap();
    format!("update {}", success)
}

async fn query_user_by_id(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match params.get("id") {
        Some(id) => {
            let id_index = id.parse::<i32>().unwrap();
            let selection = UserSelected::default();
            let entity: Option<UserSelected> = state
                .select(&selection, &UserPrimary { id: id_index })
                .await
                .unwrap();
            let json = serde_json::to_string(&entity).unwrap();
            format!("Hello, {}!", json)
        }
        None => "Missing 'id' parameter".to_string(),
    }
}

async fn query_user_by_name_and_age(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let name = params.get("name").unwrap();
    let age = params.get("age").unwrap();

    let age = age.parse::<i32>().unwrap();
    let selection = UserSelected::default();
    let location = And::new(
        UserLocation::Age(Expr::new(Cmp::Eq, age)),
        UserLocation::Name(Expr::new(Cmp::Eq, name.to_owned())),
    );
    let pagination = Pagination::new(10, 0);
    let order_by = UserOrderBy::build(vec!["id"]).unwrap();
    let entities: Vec<UserSelected> = state
        .search(&selection, &location, &order_by, &pagination)
        .await
        .unwrap();
    let json = serde_json::to_string(&entities).unwrap();
    format!("Hello, {}!", json)
}
