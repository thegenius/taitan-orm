// use crate::FieldDef;
//
//
//
// pub enum FieldGroup<'a> {
//     Optional(FieldDef<'a>),
//     Required(Vec<FieldDef<'a>>),
// }
// impl<'a> FieldGroup<'a> {
//     pub fn is_optional(&self) -> bool {
//         matches!(self, FieldGroup::Optional(_))
//     }
//     pub fn len(&self) -> usize {
//         match self {
//             Self::Optional(_) => 1,
//             Self::Required(fields) => fields.len(),
//         }
//     }
// }
//
// pub struct FieldGroupList<'a> {
//     pub groups: Vec<FieldGroup<'a>>,
//     pub first_required: usize,
//     pub is_all_required: bool,
// }
//
// impl<'a> FieldGroupList<'a> {
//     pub fn from<T>(fields: T) -> Self
//     where
//         T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
//     {
//         let mut groups = Vec::new();
//         let mut current_group = Vec::new();
//
//         for field in fields.clone().into_iter() {
//             if field.struct_field.is_optional {
//                 // 如果当前组不为空，先将其加入结果
//                 if !current_group.is_empty() {
//                     groups.push(FieldGroup::Required(current_group));
//                     current_group = Vec::new();
//                 }
//                 // 将 is_optional 为 true 的字段单独分组
//                 groups.push(FieldGroup::Optional(field.clone()));
//             } else {
//                 // 将 is_optional 为 false 的字段加入当前组
//                 current_group.push(field.clone());
//             }
//         }
//
//         // 处理最后一组
//         if !current_group.is_empty() {
//             groups.push(FieldGroup::Required(current_group));
//         }
//
//         let is_all_required = fields.into_iter().all(|f| !f.struct_field.is_optional);
//
//         let first_required = {
//             let mut first_required_index = groups.len();
//             for (index, group) in groups.iter().enumerate() {
//                 if let FieldGroup::Required(_) = group {
//                     first_required_index = index;
//                     break;
//                 }
//             }
//             first_required_index
//         };
//
//         Self {
//             groups,
//             first_required,
//             is_all_required,
//         }
//     }
// }
