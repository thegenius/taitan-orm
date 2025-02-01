作为Input的trait 
Entity特征用于insert操作
```rust
pub trait Entity<DB: Database> {
    // sql generation
    fn gen_insert_sql<'a>(&self)-> Cow<'a, str>;
    fn gen_upsert_sql<'a>(&self)-> Cow<'a, str>;
    fn gen_create_sql<'a>(&self)-> Cow<'a, str>;
    
    // sqlx arguments generation, create is same as insert
    fn add_insert_args(&self, args: &mut DB::Arguments<'_>);
    fn add_upsert_args(&self, args: &mut DB::Arguments<'_>);
    
    fn gen_insert_args(&self) -> DB::Arguments<'_>;
    fn gen_upsert_args(&self) -> DB::Arguments<'_>;
}
```

Mutation特征用于update操作
```rust
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