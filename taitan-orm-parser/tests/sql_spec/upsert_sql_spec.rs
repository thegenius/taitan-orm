use taitan_orm_parser::{DatabaseType, SqlGenerator, SqlType};

use crate::common::ExpectSql;
use crate::common::TableDefGenerator;

fn get_expected_sql_mysql() -> &'static [&'static str] {
    &[
        "let fields = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"a,b,c\") ; has_prev = true ; if self . d . is_some () { s . push_str (\",d\") ; } if self . e . is_some () { s . push_str (\",user_name\") ; } ; s } ; let marks = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"?,?,?\") ; has_prev = true ; if self . d . is_some () { s . push_str (\",?\") ; } if self . e . is_some () { s . push_str (\",?\") ; } ; s } ; let upsert_sets = { let mut s = String :: default () ; let mut has_prev = false ; if self . d . is_some () { if has_prev { s . push (',') ; } else { has_prev = true ; } s . push_str (\"d=VALUES(d)\") ; } if self . e . is_some () { if has_prev { s . push (',') ; } else { has_prev = true ; } s . push_str (\"user_name=VALUES(user_name)\") ; } ; s } ; format ! (\"INSERT INTO user ({}) VALUES({}) ON DUPLICATE KEY UPDATE {}\" , fields , marks , upsert_sets)",
        "let fields = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"name,`select`,`and`\") ; has_prev = true ; if self . age . is_some () { s . push_str (\",age\") ; } if self . primary . is_some () { s . push_str (\",user_name\") ; } ; s } ; let marks = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"?,?,?\") ; has_prev = true ; if self . age . is_some () { s . push_str (\",?\") ; } if self . primary . is_some () { s . push_str (\",?\") ; } ; s } ; let upsert_sets = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"name=VALUES(name),`select`=VALUES(`select`),`and`=VALUES(`and`)\") ; has_prev = true ; if self . age . is_some () { s . push_str (\",age=VALUES(age)\") ; } if self . primary . is_some () { s . push_str (\",user_name=VALUES(user_name)\") ; } ; s } ; format ! (\"INSERT INTO user ({}) VALUES({}) ON DUPLICATE KEY UPDATE {}\" , fields , marks , upsert_sets)"
    ]
}

fn get_expected_sql_postgres() -> &'static [&'static str] {
    &[
        "let fields = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"a,b,c\") ; has_prev = true ; if self . d . is_some () { s . push_str (\",d\") ; } if self . e . is_some () { s . push_str (\",user_name\") ; } ; s } ; let marks = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"?,?,?\") ; has_prev = true ; if self . d . is_some () { s . push_str (\",?\") ; } if self . e . is_some () { s . push_str (\",?\") ; } ; s } ; let primarys = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"a,b,c\") ; has_prev = true ; ; s } ; let upsert_sets = { let mut s = String :: default () ; let mut has_prev = false ; if self . d . is_some () { if has_prev { s . push (',') ; } else { has_prev = true ; } s . push_str (\"d=EXCLUDED.d\") ; } if self . e . is_some () { if has_prev { s . push (',') ; } else { has_prev = true ; } s . push_str (\"user_name=EXCLUDED.user_name\") ; } ; s } ; format ! (\"INSERT INTO \\\"user\\\" ({}) VALUES({}) ON CONFLICT ({}) DO UPDATE SET {}\" , fields , marks , primarys , upsert_sets)",
        "let fields = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"name,\\\"select\\\",\\\"and\\\"\") ; has_prev = true ; if self . age . is_some () { s . push_str (\",age\") ; } if self . primary . is_some () { s . push_str (\",user_name\") ; } ; s } ; let marks = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"?,?,?\") ; has_prev = true ; if self . age . is_some () { s . push_str (\",?\") ; } if self . primary . is_some () { s . push_str (\",?\") ; } ; s } ; let primarys = { let mut s = String :: default () ; let mut has_prev = false ; ; s } ; let upsert_sets = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"name=EXCLUDED.name,\\\"select\\\"=EXCLUDED.\\\"select\\\",\\\"and\\\"=EXCLUDED.\\\"and\\\"\") ; has_prev = true ; if self . age . is_some () { s . push_str (\",age=EXCLUDED.age\") ; } if self . primary . is_some () { s . push_str (\",user_name=EXCLUDED.user_name\") ; } ; s } ; format ! (\"INSERT INTO \\\"user\\\" ({}) VALUES({}) ON CONFLICT ({}) DO UPDATE SET {}\" , fields , marks , primarys , upsert_sets)"
    ]
}

fn get_expected_sql_sqlite() -> &'static [&'static str] {
    &[
        "let fields = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"a,b,c\") ; has_prev = true ; if self . d . is_some () { s . push_str (\",d\") ; } if self . e . is_some () { s . push_str (\",user_name\") ; } ; s } ; let marks = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"?,?,?\") ; has_prev = true ; if self . d . is_some () { s . push_str (\",?\") ; } if self . e . is_some () { s . push_str (\",?\") ; } ; s } ; let primarys = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"a,b,c\") ; has_prev = true ; ; s } ; let upsert_sets = { let mut s = String :: default () ; let mut has_prev = false ; if self . d . is_some () { if has_prev { s . push (',') ; } else { has_prev = true ; } s . push_str (\"d=EXCLUDED.d\") ; } if self . e . is_some () { if has_prev { s . push (',') ; } else { has_prev = true ; } s . push_str (\"user_name=EXCLUDED.user_name\") ; } ; s } ; format ! (\"INSERT INTO user ({}) VALUES({}) ON CONFLICT ({}) DO UPDATE SET {}\" , fields , marks , primarys , upsert_sets)",
        "let fields = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"name,\\\"select\\\",\\\"and\\\"\") ; has_prev = true ; if self . age . is_some () { s . push_str (\",age\") ; } if self . primary . is_some () { s . push_str (\",user_name\") ; } ; s } ; let marks = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"?,?,?\") ; has_prev = true ; if self . age . is_some () { s . push_str (\",?\") ; } if self . primary . is_some () { s . push_str (\",?\") ; } ; s } ; let primarys = { let mut s = String :: default () ; let mut has_prev = false ; ; s } ; let upsert_sets = { let mut s = String :: default () ; let mut has_prev = false ; s . push_str (\"name=EXCLUDED.name,\\\"select\\\"=EXCLUDED.\\\"select\\\",\\\"and\\\"=EXCLUDED.\\\"and\\\"\") ; has_prev = true ; if self . age . is_some () { s . push_str (\",age=EXCLUDED.age\") ; } if self . primary . is_some () { s . push_str (\",user_name=EXCLUDED.user_name\") ; } ; s } ; format ! (\"INSERT INTO user ({}) VALUES({}) ON CONFLICT ({}) DO UPDATE SET {}\" , fields , marks , primarys , upsert_sets)"
    ]
}

#[test]
fn upsert_sql_spec() {
    let table_def_generator = TableDefGenerator::new();
    table_def_generator.validate(DatabaseType::MySql, SqlType::Upsert, get_expected_sql_mysql());
    table_def_generator.validate(DatabaseType::Postgres, SqlType::Upsert, get_expected_sql_postgres());
    table_def_generator.validate(DatabaseType::Sqlite, SqlType::Upsert, get_expected_sql_sqlite());
}
