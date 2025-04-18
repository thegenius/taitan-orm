You can run `cargo bench` to bench all the ORMs.

For now, rbatis may panic when quit criterion, but bench can be completed successfully.
You can uncomment the rbatis bench config to run it.
https://github.com/rbatis/rbatis/issues/581
```toml
#[[bench]]
#name = "insert_by_rbatis"
#harness = false

#[[bench]]
#name = "select_by_rbatis"
#harness = false
```