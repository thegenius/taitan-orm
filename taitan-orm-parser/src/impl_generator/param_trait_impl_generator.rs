use crate::condition_def::VariantsOrFields;
use crate::{ConditionDef, DatabaseType, FieldMapper, TableDef};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug, Default)]
pub struct ParameterTraitImplGenerator;
impl ParameterTraitImplGenerator {
    pub fn gen_add_to_args(&self, db_type: &DatabaseType, table_def: &TableDef) -> TokenStream {
        let struct_name = &table_def.struct_name;
        let struct_ident = format_ident!("{}", &struct_name);

        let field_mapper = FieldMapper::new();
        let stream = field_mapper.gen_add_to_args(&table_def.fields);
        let db_ident = db_type.gen_ident();
        quote! {
            impl taitan_orm::traits::Parameter<sqlx::#db_ident> for #struct_ident {
                fn add_to_args<'a, 'b>(&'a self, args: &'b mut <sqlx::#db_ident as sqlx::Database>::Arguments<'a>) -> taitan_orm::result::Result<()> {
                    #stream
                    Ok(())
                }
            }
        }
    }

    pub fn gen_enum_add_to_args(
        &self,
        db_type: &DatabaseType,
        cond_def: &ConditionDef,
    ) -> TokenStream {
        let struct_name = &cond_def.struct_name;
        let struct_ident = format_ident!("{}", &struct_name);

        let variants = &cond_def.variants_or_fields;

        // match self {
        //     Spec003::Age { age } => {
        //         Arguments::add(args, age.val)?;
        //     }
        //     Spec003::AgeBirthday { age, birthday } => {
        //         Arguments::add(args, age.val)?;
        //         Arguments::add(args, birthday.val)?;
        //     }
        // }
        let field_mapper = FieldMapper::new();
        let mut stream = TokenStream::new();
        match variants {
            VariantsOrFields::Variants(variants) => {
                let mut s = TokenStream::new();
                for variant in variants {
                    let variant_name = &variant.name;
                    let variant_ident = format_ident!("{}", variant_name);
                    let fields_idents = variant
                        .fields
                        .iter()
                        .map(|field| format_ident!("{}", field.struct_field.get_name()))
                        .collect::<Vec<_>>();
                    let fields_stream = field_mapper.gen_enum_add_to_args(&variant.fields);
                    if variant.named {
                        s.extend(quote! {
                            #struct_ident::#variant_ident{ #(#fields_idents,)* } => {
                                #fields_stream
                            }
                        });
                    } else {
                        s.extend(quote! {
                            #struct_ident::#variant_ident(e0) => {
                                #fields_stream
                            }
                        });
                    }
                }
                stream.extend(s);
            }
            VariantsOrFields::Fields(fields) => panic!("unexpected fields, expected variants"),
        }

        let db_ident = db_type.gen_ident();
        quote! {
            impl taitan_orm::traits::Parameter<sqlx::#db_ident> for #struct_ident {
                fn add_to_args<'a, 'b>(&'a self, args: &'b mut <sqlx::#db_ident as sqlx::Database>::Arguments<'a>) -> taitan_orm::result::Result<()> {
                    match self {
                        #stream
                    }
                    Ok(())
                }
            }
        }
    }
}
