use taitan_orm_macro::SchemaNew;

#[derive(SchemaNew)]
#[primary(a)]
#[unique(uk_01 = (a, b))]
#[index(
   idx_01 = (a, b,c),
   idx_02 = (a, b, c,d),
)]
struct SchemaSpec001 {
    a: String,
    b: i64,
    c: bool,
    d: u64,
}
