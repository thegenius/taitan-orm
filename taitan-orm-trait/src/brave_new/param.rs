use sqlx::Database;
use crate::brave_new::result::Result;
pub trait Parameter<DB: Database> {
    fn add_to_args(&self, args: &mut <DB as Database>::Arguments<'_>) -> Result<()>;

    fn gen_args(&self) -> Result<<DB as Database>::Arguments<'_>> {
        let mut args = <DB as Database>::Arguments::default();
        self.add_to_args(&mut args)?;
        Ok(args)
    }
}

pub trait UpsertParameter<DB: Database> {
    fn add_to_upsert_args(&self, args: &mut <DB as Database>::Arguments<'_>) -> Result<()>;

    fn gen_upsert_args(&self) -> Result<<DB as Database>::Arguments<'_>> {
        let mut args = <DB as Database>::Arguments::default();
        self.add_to_upsert_args(&mut args)?;
        Ok(args)
    }
}

pub trait CountParam<DB: Database> {
    fn add_to_count_args(&self, args: &mut <DB as Database>::Arguments<'_>) -> Result<()>;

    fn gen_count_args(&self) -> Result<<DB as Database>::Arguments<'_>> {
        let mut args = <DB as Database>::Arguments::default();
        self.add_to_count_args(&mut args)?;
        Ok(args)
    }
}