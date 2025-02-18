use crate::field_mapper::CommaType;
use crate::sql_generator::{FieldGroup, FieldGroupList};
use crate::{FieldDef, KeywordsEscaper, MultiFieldMapper, SingleFieldMapper};
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
    ConditionIndexed,
}

// fn gen_comma(&self, map_type: &MapType) -> TokenStream {
//     match map_type {
//         MapType::Name => quote! { names.push(',') },
//         MapType::Mark => quote! { marks.push(',') },
//         MapType::MarkIndexed => quote! { marks.push(',') },
//         MapType::Set => quote! { sets.push(',') },
//         MapType::SetIndexed => quote! { sets.push(',') },
//         MapType::Condition => quote! { conditions.push(',') },
//         MapType::ConditionIndexed => quote! { conditions.push(',') },
//     }
// }
pub trait Connector: MultiFieldMapper {
    fn check_optional<T: AsRef<str>>(field_name: T, origin: TokenStream) -> TokenStream {
        let field_ident = format_ident!("{}", field_name.as_ref());
        quote! {
            if self.#field_ident.is_some() {
                    #origin
            }
        }
    }

    fn connect(&self, fields: &[FieldDef], escaper: &dyn KeywordsEscaper) -> TokenStream {
        let field_group_list = FieldGroupList::from(fields);
        let mut stream = TokenStream::new();
        let groups = field_group_list.groups;
        let first_required_index = field_group_list.first_required;

        for (index, group) in groups.iter().enumerate() {
            match group {
                FieldGroup::Required(fields) => {
                    if index == first_required_index {
                        let literal_payload = MultiFieldMapper::map(self, fields, escaper);
                        stream.extend(quote! {
                            s.push_str(#literal_payload);
                            has_prev = true;
                        })
                    } else {
                        let literal_payload =
                            MultiFieldMapper::map_with_leading_comma(self, fields, escaper);
                        stream.extend(quote! {
                            s.push_str(#literal_payload);
                        })
                    };
                }

                FieldGroup::Optional(field) => {
                    let field_ident = format_ident!("{}", field.struct_field.name);

                    if index < first_required_index {
                        let field_stream = SingleFieldMapper::map(self, field, escaper);
                        stream.extend(quote! {
                            if self.#field_ident.is_some() {
                                if has_prev {
                                    value_ident.push(',');
                                }
                                s.push_str(#field_stream);
                                has_prev = true;
                            }
                        });
                    } else {
                        let field_stream =
                            SingleFieldMapper::map_with_leading_comma(self, field, escaper);
                        stream.extend(quote! {
                            if self.#field_ident.is_some() {
                                s.push_str(#field_stream);
                            }
                        });
                    };
                }
            }
        }

        quote! { {
            let mut s = String::default();
            let mut has_prev = false;
            #stream;
            s
        } }
    }

    fn connect_indexed(&self, fields: &[FieldDef], escaper: &dyn KeywordsEscaper) -> TokenStream {
        let field_group_list = FieldGroupList::from(fields);
        let mut stream = TokenStream::new();
        let groups = field_group_list.groups;
        let first_required_index = field_group_list.first_required;

        for (index, group) in groups.iter().enumerate() {
            match group {
                FieldGroup::Required(fields) => {
                    if index == first_required_index {
                        let literal_payload = MultiFieldMapper::map_indexed(self, fields, escaper);
                        let len = fields.len();
                        stream.extend(quote! {
                            s.push_str(#literal_payload);
                            has_prev = true;
                            index = index + #len;
                        })
                    } else {
                        for field in fields {
                            let literal_payload =
                                SingleFieldMapper::map_indexed_dynamic_with_leading_comma(
                                    self, field, escaper,
                                );
                            stream.extend(quote! {
                                s.push_str(#literal_payload.as_ref());
                                index = index + 1;
                            })
                        }
                    };
                }

                FieldGroup::Optional(field) => {
                    let field_ident = format_ident!("{}", field.struct_field.name);

                    if index < first_required_index {
                        let field_stream =
                            SingleFieldMapper::map_indexed_dynamic(self, field, escaper);
                        stream.extend(quote! {
                            if self.#field_ident.is_some() {
                                if has_prev {
                                    value_ident.push(',');
                                }
                                s.push_str(#field_stream.as_ref());
                                has_prev = true;
                            }
                        });
                    } else {
                        let field_stream =
                            SingleFieldMapper::map_indexed_dynamic_with_leading_comma(
                                self, field, escaper,
                            );
                        stream.extend(quote! {
                            if self.#field_ident.is_some() {
                                s.push_str(#field_stream.as_ref());
                            }
                        });
                    };
                }
            }
        }

        quote! { {
            let mut s = String::default();
            let mut has_prev = false;
            let mut index = 1;
            #stream;
            s
        } }
    }
}

impl<T: MultiFieldMapper> Connector for T {}
