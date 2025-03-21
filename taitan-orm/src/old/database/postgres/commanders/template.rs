use crate::{
    execute_by_template_fn, fetch_all_by_template_fn, fetch_one_by_template_fn,
    fetch_option_by_template_fn, fetch_paged_by_template_fn
};
use crate::{SqlExecutor, SqlGenerator, SqlGeneratorContainer};

pub trait PostgresTemplateCommander:
    SqlExecutor<DB = sqlx::Postgres> + SqlGeneratorContainer
{
    execute_by_template_fn!(crate::traits::TemplateRecord::gen_template_arguments_postgres);

    fetch_one_by_template_fn!(crate::traits::TemplateRecord::gen_template_arguments_postgres);

    fetch_option_by_template_fn!(crate::traits::TemplateRecord::gen_template_arguments_postgres);

    fetch_all_by_template_fn!(crate::traits::TemplateRecord::gen_template_arguments_postgres);

    fetch_paged_by_template_fn!(
        crate::traits::TemplateRecord::gen_template_count_arguments_postgres,
        crate::traits::TemplateRecord::gen_template_arguments_postgres
    );
}
