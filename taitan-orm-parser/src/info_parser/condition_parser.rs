use crate::attr_parser::{AttrParser, NamedAttribute};
use crate::condition_def::ConditionDef;
use crate::{FieldDef, InputParser};
use case::CaseExt;
use std::borrow::Cow;
use syn::DeriveInput;

#[derive(PartialEq, Clone, Copy, Default)]
pub struct ConditionParser;

impl ConditionParser {
    pub fn parse(input: &DeriveInput) -> ConditionDef {
        let struct_name = input.ident.to_string();
        let table_name_attr: Option<NamedAttribute> = AttrParser::extract(&input.attrs, "table");
        let table_name = if let Some(attr) = &table_name_attr {
            attr.get_single_value().to_owned()
        } else {
            Cow::Owned(struct_name.to_snake())
        };
        let serde_attr: Option<NamedAttribute> = AttrParser::extract(&input.attrs, "serde_struct");
        let serde_structs = serde_attr.map(|attr| attr.values).unwrap_or_default();

        let variants = InputParser::get_enum_variant_defs(&input.data).unwrap();

        ConditionDef {
            struct_name: Cow::Owned(struct_name),
            table_name,
            variants,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{FieldName, NamedVariantDef, StructFieldDef, TableColumnDef};
    use syn::parse_quote;

    #[test]
    fn test_parse_001() {
        let input = parse_quote! {
            enum LocationSpec001 {
                A{name: LocationExpr<String>},
            }
        };
        let cond_def = ConditionParser::parse(&input);
        let expected_def = ConditionDef {
            struct_name: Cow::Borrowed("LocationSpec001"),
            table_name: Cow::Borrowed("location_spec001"),
            variants: vec![NamedVariantDef {
                name: "A".to_owned(),
                named: true,
                fields: vec![FieldDef {
                    struct_field: StructFieldDef {
                        name: FieldName::named("name"),
                        rust_type: Cow::Borrowed("LocationExpr < String >"),
                        is_optional: false,
                        is_location_expr: true,
                        is_enum_variant: true,
                        lifetime: None,
                        field: cond_def.variants.first().unwrap().fields.first().unwrap().struct_field.field.clone(),
                    },
                    table_column: TableColumnDef {
                        name: None,
                        column_type: None,
                        default_value: None,
                        generated: None,
                        nullable: false,
                        auto_inc: false,
                    },
                }],
            }],
        };
        assert_eq!(cond_def, expected_def);
    }

    #[test]
    fn test_parse_002() {
        let input = parse_quote! {
            enum LocationSpec002 {
                A(LocationExpr<String>, LocationExpr<String>),
            }
        };
        let cond_def = ConditionParser::parse(&input);
        let expected_def = ConditionDef {
            struct_name: Cow::Borrowed("LocationSpec002"),
            table_name: Cow::Borrowed("location_spec002"),
            variants: vec![NamedVariantDef {
                name: "A".to_owned(),
                named: false,
                fields: vec![
                    FieldDef {
                        struct_field: StructFieldDef {
                            name: FieldName::unnamed(0),
                            rust_type: Cow::Borrowed("LocationExpr < String >"),
                            is_optional: false,
                            is_location_expr: true,
                            is_enum_variant: true,
                            lifetime: None,
                            field: cond_def.variants.first().unwrap().fields.first().unwrap().struct_field.field.clone(),
                        },
                        table_column: TableColumnDef {
                            name: Some(Cow::Borrowed("a")),
                            column_type: None,
                            default_value: None,
                            generated: None,
                            nullable: false,
                            auto_inc: false,
                        },
                    },
                    FieldDef {
                        struct_field: StructFieldDef {
                            name: FieldName::unnamed(1),
                            rust_type: Cow::Borrowed("LocationExpr < String >"),
                            is_optional: false,
                            is_location_expr: true,
                            is_enum_variant: true,
                            lifetime: None,
                            field: cond_def.variants.first().unwrap().fields.get(1).unwrap().struct_field.field.clone(),
                        },
                        table_column: TableColumnDef {
                            name: Some(Cow::Borrowed("a")),
                            column_type: None,
                            default_value: None,
                            generated: None,
                            nullable: false,
                            auto_inc: false,
                        },
                    },
                ],
            }],
        };
        assert_eq!(cond_def, expected_def);
    }
}
