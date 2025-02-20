use crate::common::TableDefGenerator;
use taitan_orm_parser::{DatabaseType, SqlType};

fn get_expected_sql() -> &'static [&'static str] {
    &[
        "let fields = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"a,b,c\") ; has_prev = true ; if self . d . is_some () { s . push_str (\",d\") ; } if self . e . is_some () { s . push_str (\",user_name\") ; } ; s } ; let marks = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"?,?,?\") ; has_prev = true ; if self . d . is_some () { s . push_str (\",?\") ; } if self . e . is_some () { s . push_str (\",?\") ; } ; s } ; format ! (\"INSERT INTO user ({}) VALUES({})\" , fields , marks)",
        "let fields = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"name,`select`,`and`\") ; has_prev = true ; if self . age . is_some () { s . push_str (\",age\") ; } if self . primary . is_some () { s . push_str (\",user_name\") ; } ; s } ; let marks = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"?,?,?\") ; has_prev = true ; if self . age . is_some () { s . push_str (\",?\") ; } if self . primary . is_some () { s . push_str (\",?\") ; } ; s } ; format ! (\"INSERT INTO user ({}) VALUES({})\" , fields , marks)"
    ]
}
#[test]
fn insert_sql_spec() {
    let g = TableDefGenerator::new();
    g.validate(DatabaseType::MySql, SqlType::Insert, get_expected_sql());
}
