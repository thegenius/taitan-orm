use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use crate::{FieldMapper, MutationStructGenerator, SqlGenerator, TableDef};

#[derive(Clone, Debug, Default)]
pub struct OrderByStructGenerator;



impl OrderByStructGenerator {
    pub fn generate(&self, table_def: &TableDef) -> TokenStream {
        let struct_name = &table_def.struct_name;
        let struct_ident = format_ident!("{}OrderBy", struct_name);

        let field_mapper = FieldMapper::new();
        // let table_name =  field_mapper.escape(&table_def.table_name, db_type);
        let sql_generator = SqlGenerator::default();
        let fields: Vec<&str> = table_def.fields.iter().map(|f|f.struct_field.name.get_name()).collect();

        let mut uk_name_stream = TokenStream::new();
        // add primary key as unique key
        let primary_fields = table_def.get_primary_fields();
        let primary_fields_names = primary_fields.iter().map(|f|f.struct_field.name.get_name()).collect::<Vec<_>>();
        uk_name_stream.extend(quote! {
            &[#(#primary_fields_names),*],
        });
        // add unique keys
        let uk_names = table_def.get_unique_names();
        for uk_name in uk_names {
            let uk_fields = table_def.get_unique_fields(uk_name.as_ref());
            let uk_fields_names = uk_fields.iter().map(|f|f.struct_field.name.get_name()).collect::<Vec<_>>();
            uk_name_stream.extend(quote! {
                 &[#(#uk_fields_names),*],
            })
        }

        quote! {
            #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
            pub struct #struct_ident<'a> {
                fields: Vec<std::borrow::Cow<'a, str>>,
            }

            impl<'a> #struct_ident<'a> {
                fn build<I, S>(fields: I) -> std::result::Result<Self, taitan_orm::error::NotValidOrderByError>
                    where
                    I: IntoIterator<Item = S> + Clone,
                    S: AsRef<str> + Into<std::borrow::Cow<'a, str>>, // 确保每个元素可以转换为 Cow<'a, str>
                {
                    let order_by = Self::default();
                    taitan_orm::order::validate_order_by(
                        fields.clone(),
                        taitan_orm::order::OrderBy::all_fields(&order_by),
                        taitan_orm::order::OrderBy::unique_fields(&order_by),
                    )?;

                    Ok(Self {
                        fields: fields.into_iter().map(Into::into).collect(),
                    })
                }
            }

            impl<'a> taitan_orm::order::OrderBy for #struct_ident<'a> {
                fn unique_fields(&self) -> &[&[&str]] {
                    &[#uk_name_stream]
                }

                fn all_fields(&self) -> &[&str] {
                    &[#(#fields),*]
                }
                fn get_fields(&self) -> &[std::borrow::Cow<'a, str>] {
                    &self.fields
                }
            }

        }
    }
}
