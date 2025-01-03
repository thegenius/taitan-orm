This is a simple example of axum integration

# 1. create state as 
```rust
let mut shared_state =Arc::new(
    AppState::build_sqlite("./workspace", "test.db")
        .await
        .unwrap(),
);

let app = Router::new()
    .route("/user", post(create_user))
    .with_state(shared_state.clone());
```