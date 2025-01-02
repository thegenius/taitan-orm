mod state;

use crate::state::AppState;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::{routing::get, Router};
use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::Arc;
use taitan_orm::traits::Selection;
use taitan_orm::{Optional, ReaderApi, Schema, SqlExecutor, WriterApi};

#[derive(Schema, Clone, Debug)]
#[table_name = "user"]
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

    // 1. insert entity
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
        .with_state(shared_state.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn query_user_by_id(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match params.get("id") {
        Some(id) => {
            let selection = UserSelectedEntity::full_fields();
            let entity: Option<UserSelectedEntity> =
                state.select(&selection, &UserPrimary { id: 1 }).await.unwrap();
            // serde_json::to_string(&entity);

            format!("Hello, {}!", id)
        }
        None => "Missing 'id' parameter".to_string(),
    }
}
