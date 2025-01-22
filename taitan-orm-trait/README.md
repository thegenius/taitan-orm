作为Input的trait 
```rust

trait EntityInput {
    fn add_insert_args();
    fn add_upsert_args();
}

// 不同的Database应该有不同的实现
trait Entity<Database> {
    fn insert_sql();
    fn upsert_sql();
    fn create_sql();
}


trait Mutation {
    fn update_sql();
    fn update_args();
}



trait Location {
    fn where_sql();
    fn where_args();
}
trait Unique {
    fn where_sql();
    fn where_args();
}
trait TemplateRecord {
    fn template_sql();
    fn template_args();
}
```

作为output的trait
```rust
trait Selected {}
```


```rust
trait Location {
    
}
```