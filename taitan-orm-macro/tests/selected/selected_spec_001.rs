// use serde::Serialize;
// use taitan_orm_macro::{Parameter, Selected};
// use taitan_orm_trait::traits::Selected;
//
// #[derive(Debug, Selected)]
// struct SelectedSpec001 {
//     a: Option<Option<String>>,
//     b: Option<Option<i64>>,
// }
//
//
// #[test]
// fn test_selected_spec_001() {
//     let entity = SelectedSpec001 {
//         a: Some(None),
//         b: Some(None),
//     };
//     let selected_sql = Selected::<sqlx::Postgres>::gen_select_sql(&entity);
//     assert_eq!(selected_sql, "a,b".to_string());
//
//     let entity = SelectedSpec001 {
//         a: None,
//         b: Some(None),
//     };
//     let selected_sql = Selected::<sqlx::Postgres>::gen_select_sql(&entity);
//     assert_eq!(selected_sql, "b".to_string());
// }
