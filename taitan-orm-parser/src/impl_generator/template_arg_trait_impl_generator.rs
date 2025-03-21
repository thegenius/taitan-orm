use crate::{DatabaseType, FieldMapper, TableDef};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug, Default)]
pub struct TemplateArgTraitImplGenerator;
impl TemplateArgTraitImplGenerator {
    pub fn gen_add_to_args(
        &self,
        db_type: &DatabaseType,
        table_def: &TableDef,
    ) -> TokenStream {
        let struct_name = &table_def.struct_name;
        let struct_ident = format_ident!("{}", &struct_name);

        let field_mapper = FieldMapper::new();
        let stream = field_mapper.gen_template_add_to_args(&table_def.fields);
        let db_ident = db_type.gen_ident();
        quote! {
            impl taitan_orm_trait::traits::TemplateArgTrait<sqlx::#db_ident> for #struct_ident {
                fn add_to_args<'a, 'b>(&'a self, name: &'b str, args: &'b mut <sqlx::#db_ident as sqlx::Database>::Arguments<'a>) -> taitan_orm_trait::result::Result<()> {
                    #stream
                    Ok(())
                }
            }
        }
    }
}
