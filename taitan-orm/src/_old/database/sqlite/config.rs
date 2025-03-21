use std::borrow::Cow;

pub struct SqliteLocalConfig<'a> {
    pub work_dir: Cow<'a, str>,
    pub db_file: Cow<'a, str>,
}

impl<'a> SqliteLocalConfig<'a> {
    pub fn new<S>(work_dir: S, db_file: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            work_dir: work_dir.into(),
            db_file: db_file.into(),
        }
    }
}
