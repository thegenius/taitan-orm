use crate::{
    execute_by_template_fn, fetch_all_by_template_fn, fetch_one_by_template_fn,
    fetch_option_by_template_fn, fetch_paged_by_template_fn
};
use crate::{SqlExecutor, SqlGenerator, SqlGeneratorContainer};


pub trait MySqlTemplateCommander: SqlExecutor<DB = sqlx::MySql> + SqlGeneratorContainer {
    execute_by_template_fn!(crate::traits::TemplateRecord::gen_template_arguments_mysql);

    fetch_one_by_template_fn!(crate::traits::TemplateRecord::gen_template_arguments_mysql);

    fetch_option_by_template_fn!(crate::traits::TemplateRecord::gen_template_arguments_mysql);

    fetch_all_by_template_fn!(crate::traits::TemplateRecord::gen_template_arguments_mysql);

    fetch_paged_by_template_fn!(
        crate::traits::TemplateRecord::gen_template_count_arguments_mysql,
        crate::traits::TemplateRecord::gen_template_arguments_mysql
    );
}
