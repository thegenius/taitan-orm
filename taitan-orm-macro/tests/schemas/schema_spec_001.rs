use serde::Deserialize;

use taitan_orm_macro::Schema;

#[derive(Schema)]
#[primary(a)]
#[unique(uk_01 = (a, b))]
#[index(
   idx_01 = (a, b, c),
   idx_02 = (a, b, c,d),
)]
#[derive(Debug)]
struct SchemaSpec001 {
    a: String,
    b: i64,
    c: bool,
    d: i64,
}

