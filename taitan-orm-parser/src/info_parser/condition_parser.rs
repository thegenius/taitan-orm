use crate::attr_parser::{AttrParser, NamedAttribute};
use crate::condition_def::{ConditionDef, VariantsOrFields};
use crate::{FieldDef, InputParser};
use case::CaseExt;
use std::borrow::Cow;
use syn::parse::Parser;
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

        let is_enum = InputParser::is_enum(&input.data);
        let variants_or_fields = if is_enum {
            let variants = InputParser::get_enum_variant_defs(&input.data).unwrap();
            VariantsOrFields::Variants(variants)
        } else {
            let fields = InputParser::get_fields(&input.data);
            let defs = fields
                .iter()
                .map(|v| FieldDef::parse(v, false, None, None))
                .collect();
            VariantsOrFields::Fields(defs)
        };

        ConditionDef {
            struct_name: Cow::Owned(struct_name),
            table_name,
            variants_or_fields,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{FieldName, NamedVariantDef, ParsedField, TableColumnDef};
    use syn::parse_quote;

    #[test]
    fn test_parse_001() {
        let input = parse_quote! {
            enum LocationSpec001 {
                A{name: LocationExpr<String>},
            }
        };

        let cond_def = ConditionParser::parse(&input);
        let field = match &cond_def.variants_or_fields {
            VariantsOrFields::Variants(v) => v
                .first()
                .unwrap()
                .fields
                .first()
                .unwrap()
                .struct_field
                .origin_field
                .clone(),
            _ => unreachable!(),
        };
        let variant = NamedVariantDef {
            name: "A".to_owned(),
            named: true,
            fields: vec![FieldDef {
                struct_field: ParsedField {
                    name: FieldName::named("name"),
                    rust_type: Cow::Borrowed("LocationExpr < String >"),
                    option_nest_level: 0,
                    is_location_expr: true,
                    is_enum_variant: true,
                    lifetime: None,
                    origin_field: field,
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
        };
        let expected_def = ConditionDef {
            struct_name: Cow::Borrowed("LocationSpec001"),
            table_name: Cow::Borrowed("location_spec001"),
            variants_or_fields: VariantsOrFields::Variants(vec![variant]),
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

        let field = match &cond_def.variants_or_fields {
            VariantsOrFields::Variants(v) => v
                .first()
                .unwrap()
                .fields
                .first()
                .unwrap()
                .struct_field
                .origin_field
                .clone(),
            _ => unreachable!(),
        };
        let field2 = match &cond_def.variants_or_fields {
            VariantsOrFields::Variants(v) => v
                .first()
                .unwrap()
                .fields
                .get(1)
                .unwrap()
                .struct_field
                .origin_field
                .clone(),
            _ => unreachable!(),
        };
        let variant1 = NamedVariantDef {
            name: "A".to_owned(),
            named: false,
            fields: vec![
                FieldDef {
                    struct_field: ParsedField {
                        name: FieldName::unnamed(0),
                        rust_type: Cow::Borrowed("LocationExpr < String >"),
                        option_nest_level: 0,
                        is_location_expr: true,
                        is_enum_variant: true,
                        lifetime: None,
                        origin_field: field,
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
                    struct_field: ParsedField {
                        name: FieldName::unnamed(1),
                        rust_type: Cow::Borrowed("LocationExpr < String >"),
                        option_nest_level: 0,
                        is_location_expr: true,
                        is_enum_variant: true,
                        lifetime: None,
                        origin_field: field2,
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
        };


        let expected_def = ConditionDef {
            struct_name: Cow::Borrowed("LocationSpec002"),
            table_name: Cow::Borrowed("location_spec002"),
            variants_or_fields: VariantsOrFields::Variants(vec![variant1]),
        };
        assert_eq!(cond_def, expected_def);
    }
}
