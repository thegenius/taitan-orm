use crate::{DatabaseType, FieldMapper, TableDef};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug, Default)]
pub struct ParameterTraitImplGenerator;
impl ParameterTraitImplGenerator {
    pub fn gen_add_to_args(
        &self,
        db_type: &DatabaseType,
        table_def: &TableDef,
    ) -> TokenStream {
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
}
