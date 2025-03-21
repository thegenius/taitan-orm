// use time::PrimitiveDateTime;
// use uuid::Uuid;
//
// #[derive(taitan_orm :: prelude :: Condition, Default, Debug, Clone)]
// #[table_name = "user"]
// pub struct UserLocation {
//     mode: taitan_orm::prelude::LocationMode,
//     pub id: taitan_orm::result::Optional<taitan_orm::traits::LocationExpr<i64>>,
//     pub request_id: taitan_orm::result::Optional<taitan_orm::traits::LocationExpr<Uuid>>,
//     pub age: taitan_orm::result::Optional<taitan_orm::traits::LocationExpr<i32>>,
//     pub name: taitan_orm::result::Optional<taitan_orm::traits::LocationExpr<String>>,
//     pub birthday: taitan_orm::result::Optional<taitan_orm::traits::LocationExpr<PrimitiveDateTime>>,
// }