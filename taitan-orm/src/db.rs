use crate::extractor::Extractor;
use crate::prelude::{
    ReaderApi, SqlExecutor, SqlGeneratorContainer, SqlGenericExecutor, TemplateApi, WriterApi,
};

pub trait Executor:
    ReaderApi
    + WriterApi
    + TemplateApi
    + Extractor
    + SqlExecutor
    + SqlGenericExecutor
    + SqlGeneratorContainer
{
}

impl<T> Executor for T where
    T: ReaderApi
        + WriterApi
        + TemplateApi
        + Extractor
        + SqlExecutor
        + SqlGenericExecutor
        + SqlGeneratorContainer
{
}

// #[derive(Debug, Clone)]
// pub struct DB<
//     T: Executor,
// >(pub T);
//
// impl<T> Deref for DB<T>
// where
//     T: Executor
// {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
//
// impl<T> DerefMut for DB<T>
// where
//     T: Executor,
// {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }
