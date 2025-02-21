use super::field_group_list::{FieldGroup, FieldGroupList};
use super::{KeywordsEscaper, MultiFieldMapper, SingleFieldMapper};
use crate::FieldDef;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::borrow::Cow;
use std::os::unix::raw::mode_t;

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

// pub enum MapType {
//     Name,
//     Mark,
//     MarkIndexed,
//     Set,
//     SetIndexed,
//     Condition,
//     ConditionIndexed,
// }

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

enum StreamType {
    Required,
    RequiredCheckPrev,
    RequiredLeadingComma,
    Optional,
    OptionalCheckPrev,
    OptionalLeadingComma,
}

impl StreamType {
    pub fn parse_dynamic(
        field: &FieldDef,
        group_index: usize,
        index: usize,
        first_required_group_index: usize,
    ) -> Self {
        if !field.struct_field.is_optional {
            if index == 0 && group_index == 0 {
                assert_eq!(first_required_group_index, 0);
                return StreamType::Required;
            }
            if index == 0 && group_index <= first_required_group_index {
                assert_eq!(group_index, first_required_group_index);
                assert_ne!(group_index, 0);
                return StreamType::RequiredCheckPrev;
            }
            StreamType::RequiredLeadingComma
        } else {
            if index == 0 && group_index == 0 {
                assert_ne!(first_required_group_index, 0);
                return StreamType::Optional;
            }
            if group_index < first_required_group_index {
                StreamType::OptionalCheckPrev
            } else {
                assert_ne!(group_index, first_required_group_index);
                StreamType::OptionalLeadingComma
            }
        }
    }

    pub fn parse_static(
        field: &FieldDef,
        group_index: usize,
        index: usize,
        first_required_group_index: usize,
    ) -> Self {
        assert!(!field.struct_field.is_optional);
        assert_eq!(group_index, 0);
        if index == 0 && group_index == 0 {
            assert_eq!(first_required_group_index, 0);
            return StreamType::Required;
        }
        if index == 0 && group_index <= first_required_group_index {
            assert_eq!(group_index, first_required_group_index);
            assert_ne!(group_index, 0);
            return StreamType::RequiredCheckPrev;
        }
        StreamType::RequiredLeadingComma
    }

    // pub fn transform<T: AsRef<str>>(&self, field_name: T, origin: TokenStream, indexed: bool) -> TokenStream {
    //     match self {
    //         StreamType::Required => quote! {
    //             s.push_str(#origin);
    //         }
    //         StreamType::RequiredCheckPrev => quote! {
    //             if has_prev {
    //                 s.push(',');
    //             } else {
    //                 has_prev = true;
    //             }
    //             s.push_str(#origin);
    //         }
    //         StreamType::RequiredLeadingComma => quote! {
    //             s.push_str(#origin);
    //         }
    //         StreamType::Optional => {
    //             let field_ident = format_ident!("{}", field_name.as_ref());
    //             quote! {
    //                 if self.#field_ident.is_some() {
    //                     s.push_str(#origin);
    //                 }
    //             }
    //         }
    //         StreamType::OptionalCheckPrev => {
    //             let field_ident = format_ident!("{}", field_name.as_ref());
    //             quote! {
    //                 if self.#field_ident.is_some() {
    //                     s.push_str(#origin);
    //                 }
    //             }
    //         }
    //         StreamType::OptionalLeadingComma => {
    //             let field_ident = format_ident!("{}", field_name.as_ref());
    //             quote! {
    //                 if self.#field_ident.is_some() {
    //                     s.push_str(#origin);
    //                 }
    //             }
    //         }
    //     }
    // }
}

pub trait Connector: MultiFieldMapper {
    fn check_optional<T: AsRef<str>>(field_name: T, origin: TokenStream) -> TokenStream {
        let field_ident = format_ident!("{}", field_name.as_ref());
        quote! {
            if self.#field_ident.is_some() {
                    #origin
            }
        }
    }


    // fn extend_stream<T: AsRef<str>>(dest: &mut TokenStream, field_name: T, check_optional: bool, origin: TokenStream) {
    //
    //     let s = if check_optional {
    //         let field_ident = format_ident!("{}", field_name.as_ref());
    //         quote! {
    //             if self.#field_ident.is_some() {
    //                 s.push_str(#origin.as_ref());
    //             }
    //         }
    //     } else {
    //         origin
    //     };
    // }

    fn connect<'a, T>(&self, fields: T, escaper: &dyn KeywordsEscaper) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        let field_group_list = FieldGroupList::from(fields);
        let mut stream = TokenStream::new();
        let groups = field_group_list.groups;
        let first_required_index = field_group_list.first_required;

        for (group_index, group) in groups.iter().enumerate() {
            match group {
                FieldGroup::Required(fields) => {
                    if group_index == 0 {
                        let literal_payload = MultiFieldMapper::map(self, fields, escaper);
                        stream.extend(quote! {
                            s.push_str(#literal_payload);
                            has_prev = true;
                        })
                    } else if group_index == first_required_index {
                        // 因为index != 0，所以前面一定有optional的字段
                        let literal_payload = MultiFieldMapper::map(self, fields, escaper);
                        stream.extend(quote! {
                            if has_prev {
                                s.push(',')
                            } else {
                                has_prev = true;
                            }
                            s.push_str(#literal_payload);
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
                    stream.extend(self.transform_dynamic(field, escaper, false, 0, group_index, first_required_index));
                    // let field_ident = format_ident!("{}", field.struct_field.name);
                    //
                    // if index < first_required_index {
                    //     let field_stream = SingleFieldMapper::map(self, field, escaper);
                    //     stream.extend(quote! {
                    //         if self.#field_ident.is_some() {
                    //             if has_prev {
                    //                 s.push(',');
                    //             } else {
                    //                 has_prev = true;
                    //             }
                    //             s.push_str(#field_stream);
                    //         }
                    //     });
                    // } else {
                    //     let field_stream =
                    //         SingleFieldMapper::map_with_leading_comma(self, field, escaper);
                    //     stream.extend(quote! {
                    //         if self.#field_ident.is_some() {
                    //             s.push_str(#field_stream);
                    //         }
                    //     });
                    // };
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

    fn connect_indexed<'a, T>(&self, fields: T, escaper: &dyn KeywordsEscaper) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        let field_group_list = FieldGroupList::from(fields);
        let mut stream = TokenStream::new();
        let groups = field_group_list.groups;
        let first_required_index = field_group_list.first_required;

        for (index, group) in groups.iter().enumerate() {
            match group {
                FieldGroup::Required(fields) => {
                    if index == 0 {
                        let literal_payload = MultiFieldMapper::map_indexed(self, fields, escaper);
                        let len = fields.len();
                        stream.extend(quote! {
                            s.push_str(#literal_payload);
                            has_prev = true;
                            index = index + #len;
                        })
                    } else if index == first_required_index {
                        // 前面有optional字段
                        for field in fields {
                            let literal_payload =
                                SingleFieldMapper::map_dynamic_indexed_with_leading_comma(
                                    self, field, escaper,
                                );
                            stream.extend(quote! {
                                if has_prev {
                                    s.push(',');
                                } else {
                                    has_prev = true;
                                }
                                s.push_str(#literal_payload.as_ref());
                                index = index + 1;
                            })
                        }
                    } else {
                        for field in fields {
                            let literal_payload =
                                SingleFieldMapper::map_dynamic_indexed_with_leading_comma(
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
                            SingleFieldMapper::map_dynamic_indexed(self, field, escaper);
                        stream.extend(quote! {
                            if self.#field_ident.is_some() {
                                if has_prev {
                                    s.push(',');
                                } else {
                                    has_prev = true;
                                }
                                s.push_str(#field_stream.as_ref());
                                index = index + 1;
                            }
                        });
                    } else {
                        let field_stream =
                            SingleFieldMapper::map_dynamic_indexed_with_leading_comma(
                                self, field, escaper,
                            );
                        stream.extend(quote! {
                            if self.#field_ident.is_some() {
                                s.push_str(#field_stream.as_ref());
                                index = index + 1;
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

    fn connect_dynamic<'a, T>(&self, fields: T, escaper: &dyn KeywordsEscaper) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        let field_group_list = FieldGroupList::from(fields);
        let mut stream = TokenStream::new();
        let groups = field_group_list.groups;
        let first_required_index = field_group_list.first_required;

        for (index, group) in groups.iter().enumerate() {
            match group {
                FieldGroup::Required(fields) => {
                    if index == first_required_index {
                        let s = fields
                            .iter()
                            .enumerate()
                            .map(|(idx, f)| {
                                if idx == 0 {
                                    let d = SingleFieldMapper::map_dynamic(self, f, escaper);
                                    quote! { s.push_str(#d.as_ref()); }
                                } else {
                                    let d = SingleFieldMapper::map_dynamic_with_leading_comma(
                                        self, f, escaper,
                                    );
                                    quote! { s.push_str(#d.as_ref()); }
                                }
                            })
                            .collect::<Vec<_>>();
                        if index == 0 {
                            stream.extend(quote! {
                                #(#s)*
                                has_prev = true;
                            })
                        } else {
                            // 前面有optional字段
                            stream.extend(quote! {
                                if has_prev {
                                    s.push(',');
                                } else {
                                    has_prev = true;
                                }
                                #(#s)*
                            })
                        }
                    } else {
                        let s = fields
                            .iter()
                            .map(|f| {
                                let d = SingleFieldMapper::map_dynamic_with_leading_comma(
                                    self, f, escaper,
                                );
                                quote! { s.push_str(#d.as_ref()); }
                            })
                            .collect::<Vec<_>>();
                        stream.extend(quote! {
                            #(#s)*
                        })
                    };
                }

                FieldGroup::Optional(field) => {
                    let field_ident = format_ident!("{}", field.struct_field.name);
                    if index < first_required_index {
                        let field_stream = SingleFieldMapper::map_dynamic(self, field, escaper);
                        stream.extend(quote! {
                            if self.#field_ident.is_some() {
                                if has_prev {
                                    s.push(',');
                                } else {
                                    has_prev = true;
                                }
                                s.push_str(#field_stream.as_ref());
                            }
                        });
                    } else {
                        let field_stream =
                            SingleFieldMapper::map_dynamic_with_leading_comma(self, field, escaper);
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
            #stream;
            s
        } }
    }

    fn connect_dynamic_indexed<'a, T>(
        &self,
        fields: T,
        escaper: &dyn KeywordsEscaper,
    ) -> TokenStream
    where
        T: IntoIterator<Item = &'a FieldDef<'a>> + Clone,
    {
        let field_group_list = FieldGroupList::from(fields);
        let mut stream = TokenStream::new();
        let groups = field_group_list.groups;
        let first_required_index = field_group_list.first_required;

        for (index, group) in groups.iter().enumerate() {
            match group {
                FieldGroup::Required(fields) => {
                    let len = fields.len();
                    let s = fields
                        .iter()
                        .enumerate()
                        .map(|(idx, f)| {
                            if idx == 0 {
                                let d = SingleFieldMapper::map_dynamic_indexed(self, f, escaper);
                                quote! { s.push_str(#d.as_ref()); index = index + 1; }
                            } else {
                                let d = SingleFieldMapper::map_dynamic_indexed_with_leading_comma(
                                    self, f, escaper,
                                );
                                quote! { s.push_str(#d.as_ref()); index = index + 1; }
                            }
                        })
                        .collect::<Vec<_>>();

                    if index == 0 {
                        stream.extend(quote! {
                            #(#s)*
                            has_prev = true;
                        })
                    } else if index == first_required_index {
                        // 前面有optional字段
                        stream.extend(quote! {
                            if has_prev {
                                s.push(',');
                            } else {
                                has_prev = true;
                            }
                            #(#s)*
                        })
                    } else {
                        stream.extend(quote! {
                            #(#s)*
                        })
                    };
                }

                FieldGroup::Optional(field) => {
                    let field_ident = format_ident!("{}", field.struct_field.name);
                    if index < first_required_index {
                        let field_stream =
                            SingleFieldMapper::map_dynamic_indexed(self, field, escaper);
                        stream.extend(quote! {
                            if self.#field_ident.is_some() {
                                if has_prev {
                                    s.push(',');
                                } else {
                                    has_prev = true;
                                }
                                s.push_str(#field_stream.as_ref());
                                index = index + 1;
                            }
                        });
                    } else {
                        let field_stream =
                            SingleFieldMapper::map_dynamic_indexed_with_leading_comma(
                                self, field, escaper,
                            );
                        stream.extend(quote! {
                            if self.#field_ident.is_some() {
                                s.push_str(#field_stream.as_ref());
                                index = index + 1;
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
