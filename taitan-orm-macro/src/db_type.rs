use syn::DeriveInput;
use taitan_orm_parser::attr_parser::AttrParser;
use taitan_orm_parser::DatabaseType;

pub fn parse_database_type(derive_input: &DeriveInput) -> DatabaseType {
    // 遍历属性，找到 `Parameter` 属性
    for attr in &derive_input.attrs {
        if attr.path().is_ident("Parameter") {
            let attr_val = AttrParser::parse(attr).expect("expect a database type attribute");
            let db_type_str = attr_val.values.first().unwrap();
            // 解析属性内容
            if db_type_str == "MySql" {
                return DatabaseType::MySql;
            } else if db_type_str == "Sqlite" {
                return DatabaseType::Sqlite;
            } else if db_type_str == "Postgres" {
                return DatabaseType::Postgres;
            } else {
                panic!("unknown database type: {}", db_type_str);
            }
        }
    }
    panic!("no database type found");
}