#[derive(Debug)]
pub struct UserCustomTemplate {
    name: String,
    age: Option<i32>,
}
const _ : () =
    {
        extern crate askama; impl askama::Template for UserCustomTemplate
    {
        fn
        render_into_with_values<AskamaW>(&self,__askama_writer: &mut
        AskamaW,__askama_values: &dyn askama::Values) ->
                                         askama::Result<()>where AskamaW: askama::helpers::core::fmt::Write +
        ?askama::helpers::core::marker::Sized{
            #[allow(unused_imports)]use
            askama::{
                filters::{
                    AutoEscape as _, WriteWritable as _
                },helpers::{ ResultConverter as _, core::fmt::Write as _ },
            };__askama_writer.write_str("select `id`, `name`, `age` FROM `user` where ")?;if
            askama::helpers::as_bool(&(self.age.is_some())){
                __askama_writer.write_str(" age >= :{age} AND ")?;
            }__askama_writer.write_str(" `name` = :{name}")?;askama::Result::Ok(())
        }const SIZE_HINT: askama::helpers::core::primitive::usize = 81usize;
    }/// Implement the [`format!()`][askama::helpers::std::format] trait for [`UserCustomTemplate`]
    ///
    /// Please be aware of the rendering performance notice in the [`Template`][askama::Template] trait.
    impl askama::helpers::core::fmt::Display for UserCustomTemplate
    {
        #[inline]fn
        fmt(&self,f: &mut askama::helpers::core::fmt::Formatter<'_>) ->
        askama::helpers::core::fmt::Result
        {
            askama::Template::render_into(self,
                                          f).map_err(|_| askama::helpers::core::fmt::Error)
        }
    }impl askama::FastWritable for UserCustomTemplate
    {
        #[inline]fn
        write_into<AskamaW>(&self,dest: &mut AskamaW,values: &dyn
        askama::Values) -> askama::Result<()> where AskamaW:
        askama::helpers::core::fmt::Write +
        ?askama::helpers::core::marker::Sized,{
            askama::Template::render_into_with_values(self, dest, values)
        }
    }
    }; impl taitan_orm :: traits :: Template < sqlx :: Sqlite > for
UserCustomTemplate
{
    fn get_sql(& self,) -> taitan_orm :: result :: Result <
        (String, < sqlx :: Sqlite as sqlx :: Database > :: Arguments < '_ >) >
    {
        taitan_orm :: traits :: TemplateRenderTrait :: < sqlx :: Sqlite > ::
        gen_sql(self)
    } fn get_paged_sql < 'a >
(& 'a self, pagination : & 'a taitan_orm :: page :: Pagination,) ->
    taitan_orm :: result :: Result <
        (String, < sqlx :: Sqlite as sqlx :: Database > :: Arguments < 'a >) >
{
    taitan_orm :: traits :: TemplateRenderTrait :: < sqlx :: Sqlite > ::
    gen_paged_sql(self, pagination)
} fn get_count_sql(& self,) -> taitan_orm :: result :: Result <
    (String, < sqlx :: Sqlite as sqlx :: Database > :: Arguments < '_ >) >
{
    taitan_orm :: traits :: TemplateRenderTrait :: < sqlx :: Sqlite > ::
    gen_count_sql(self)
}
} impl taitan_orm :: traits :: TemplateSqlTrait for UserCustomTemplate
{
    fn get_rendered_sql(& self) -> taitan_orm :: result :: Result < String >
    {
        Ok(askama::Template::render(self).map_err(| err |
            {
                taitan_orm :: error :: TaitanOrmError ::
                TemplateRenderError(taitan_orm :: error ::
                TemplateRenderError(err.to_string()))
            }) ?)
    }
} impl taitan_orm :: traits :: TemplateArgTrait < sqlx :: Sqlite > for
UserCustomTemplate
{
    fn add_to_args < 'a, 'b >
    (& 'a self, name : & 'b str, args : & 'b mut < sqlx :: Sqlite as sqlx ::
    Database > :: Arguments < 'a >) -> taitan_orm :: result :: Result < () >
    {
        match name
        {
            "name" => sqlx :: Arguments :: add(args, & self.name) ? , "age" =>
            if let Some(f) = & self.age
            { sqlx :: Arguments :: add(args, f) ? }, _ => unreachable! (),
        } Ok(())
    }
}



#[test]
pub fn test() {

}