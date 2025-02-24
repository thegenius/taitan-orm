use taitan_orm_macro::{LocationNew, Parameter};
use taitan_orm_trait::{LocationExpr, Optional};

#[derive(Debug, Parameter, LocationNew)]
struct ParamSpec001 {
    a: LocationExpr<String>,
    b: Optional<LocationExpr<i64>>,
}

// impl taitan_orm_trait::brave_new::location::Location<sqlx::Sqlite> for ParamSpec001 {
//     fn table_name(&self) -> std::borrow::Cow<'static, str> {
//         std::borrow::Cow::Borrowed("param_spec001")
//     }
//     fn gen_where_sql<'a>(&self) -> std::borrow::Cow<'a, str> {
//         let s = {
//             let mut s = String::default();
//             let mut has_prev = false;
//             let mut index = 1;
//             s.push_str(format!("a{}${}", self.a.get_cmp_sql(), index).as_ref());
//             index = index + 1;
//             has_prev = true;
//             s.push_str(match &self.b {
//                 Optional::Some(b) => {
//                     format!(",b{}${}", b.get_cmp_sql(), index)
//                 },
//                 Optional::Null=> {
//                     format!(",b=${}",  index)
//                 }
//                 Optional::None => {
//                     "".to_string()
//                 }
//             }.as_ref());
//             s
//         };
//         std::borrow::Cow::Owned(s)
//     }
//     fn all_none(&self) -> bool {
//         false
//     }
// }
