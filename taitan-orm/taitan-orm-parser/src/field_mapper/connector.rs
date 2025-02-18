use crate::field_mapper::CommaType;
use crate::sql_generator::{FieldGroup, FieldGroupList};
use crate::FieldDef;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

// field 支持3中映射
// (1) leading required group 映射
// (2) trailing required not indexed group 映射
// (3) trailing required index 单个映射
// (4) optional


// 将field list切割为3个区间
// (1) leading required是开头处的required字段
// 这部分单独出来是因为可以进行非常多的compile time优化
// 1.1 这个区间可以视为一个整体，内部用逗号连接，编译期完成
// 1.2 整个区间结束，添加has_prev = true
// 1.3 整个区间结束，添加index = index + len
//
// (2) optional区间
// 1.1 每个字段单独处理
// 1.2 如果小于first_required，判断has_prev，添加has_prev=true
// 1.3 如果大于first_required，不判断has_prev，不添加has_prev
// 1.4 每个字段添加 index = index + 1
//
// (3) 后续的required
// 1.1 不需要index的情况下，可以区间一起处理
// 1.2 需要index的情况下，需要单独处理
// 1.3 不需要添加has_prev和判断has_prev
// 1.4 单独处理时，每个字段添加 index = index + 1

pub enum MapType {
    Name,
    Mark,
    MarkIndexed,
    Set,
    SetIndexed,
    Condition,
    ConditionIndexed
}

// pub trait Connector {
//     fn map_required_group(&self, field: &[FieldDef]) -> TokenStream;
//     fn map_optional(&self, field: &FieldDef) -> TokenStream;
//     fn gen_comma(&self, map_type: &MapType) -> TokenStream {
//         match map_type {
//             MapType::Name => quote! { names.push(',') },
//             MapType::Mark => quote! { marks.push(',') },
//             MapType::MarkIndexed => quote! { marks.push(',') },
//             MapType::Set => quote! { sets.push(',') },
//             MapType::SetIndexed => quote! { sets.push(',') },
//             MapType::Condition => quote! { conditions.push(',') },
//             MapType::ConditionIndexed => quote! { conditions.push(',') },
//         }
//     }
//
//     fn check_optional<T: AsRef<str>>(
//         field_name: T,
//         origin: TokenStream,
//     ) -> TokenStream {
//         let field_ident = format_ident!("{}", field_name.as_ref());
//         quote! {
//             if self.#field_ident.is_some() {
//                     #origin
//             }
//         }
//     }
//
//
//     fn connect(&self, fields: &[FieldDef]) -> TokenStream {
//         let field_group_list = FieldGroupList::from(fields);
//         let mut stream = TokenStream::new();
//         let groups = field_group_list.groups;
//         let first_required_index = field_group_list.first_required;
//         for (index, group) in groups.iter().enumerate() {
//             match group {
//                 FieldGroup::Required(fields) => {
//                     if index == first_required_index {
//                         stream.extend(self.map_required_group(fields));
//                     } else {
//                         stream.extend(self.connect_trailing_required_group(fields));
//                     }
//                 }
//
//                 FieldGroup::Optional(field) => {
//                     let comma_stream = if index < first_required_index {
//                         self.gen_comma()
//                     } else {
//                         self.gen_leading_comma()
//                     };
//                     for field in fields {
//                         let field_stream = self.map_optional(field);
//                         stream.extend(quote! {
//                             #comma_stream
//                             #field_stream
//                         });
//                     }
//                 }
//             }
//         }
//         stream
//     }
//
//     fn connect_indexed(&self, fields: &[FieldDef]) -> TokenStream {
//         let field_group_list = FieldGroupList::from(fields);
//         let mut stream = TokenStream::new();
//         let groups = field_group_list.groups;
//         let first_required_index = field_group_list.first_required;
//         for (index, group) in groups.iter().enumerate() {
//             match group {
//                 FieldGroup::Required(fields) => {
//                     if index == first_required_index {
//                         stream.extend(self.map_required_group(fields));
//                         let len = fields.len();
//                         stream.extend(quote! {
//                             index = index + #len;
//                         });
//                     } else {
//                         let comma_stream = self.gen_leading_comma();
//                         for field in fields {
//                             let field_stream = self.map_optional(field);
//                             stream.extend(quote! {
//                                 #comma_stream
//                                 #field_stream
//                             });
//                         }
//                     }
//                 }
//
//                 FieldGroup::Optional(field) => {
//                     let comma_stream = if index < first_required_index {
//                         self.gen_comma()
//                     } else {
//                         self.gen_leading_comma()
//                     };
//                     for field in fields {
//                         let field_stream = self.map_optional(field);
//                         stream.extend(quote! {
//                             #comma_stream
//                             #field_stream
//                         });
//                     }
//                 }
//             }
//         }
//         stream
//     }
//
// }
