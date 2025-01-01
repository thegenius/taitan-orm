use std::ops::{Deref, DerefMut};
use crate::{ReaderApi, SqlExecutor, SqlGeneratorContainer, SqlGenericExecutor, TemplateApi};
use crate::extractor::Extractor;

#[derive(Debug, Clone)]
pub struct DB<T: ReaderApi + ReaderApi + TemplateApi + Extractor + SqlExecutor + SqlGenericExecutor + SqlGeneratorContainer>(pub T);

impl<T> Deref for DB<T>
where
    T: ReaderApi + ReaderApi + TemplateApi + Extractor + SqlExecutor + SqlGenericExecutor + SqlGeneratorContainer,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for DB<T>
where
    T: ReaderApi + ReaderApi + TemplateApi + Extractor + SqlExecutor + SqlGenericExecutor + SqlGeneratorContainer,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
