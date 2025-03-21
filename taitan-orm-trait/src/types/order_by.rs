use crate::error::NotValidOrderByError;
use std::borrow::Cow;
use std::error::Error;
use std::fmt::Debug;

pub fn validate_order_by<'a, I, S>(
    fields: I,
    all_fields: &[&str],
    unique_fields_vec: &[&[&str]],
) -> Result<(), NotValidOrderByError>
where
    I: IntoIterator<Item = S> + Clone,
    S: AsRef<str>, // 确保每个元素可以转换为 &str
{
    let field_valid = fields.clone().into_iter().all(|field| {all_fields.contains(&field.as_ref())});
    if !field_valid {
        return Err(NotValidOrderByError("contains invalid field".to_owned()));
    }

    let valid = unique_fields_vec.iter().any(|unique_fields| {
        unique_fields.iter().all(|unique_field| {
            fields
                .clone()
                .into_iter()
                .any(|field| field.as_ref() == *unique_field)
        })
    });
    if valid {
        Ok(())
    } else {
        Err(NotValidOrderByError(
            "order by fields must include unique key".to_string(),
        ))
    }
}

pub trait OrderBy: Sync + Debug {
    fn unique_fields(&self) -> &[&[&str]];

    fn all_fields(&self) -> &[&str];

    fn get_fields(&self) -> &[Cow<'_, str>];
}

impl<T: OrderBy + Debug> OrderBy for &T {
    fn unique_fields(&self) -> &[&[&str]] {
        (*self).unique_fields()
    }

    fn all_fields(&self) -> &[&str] {(*self).all_fields()}

    fn get_fields(&self) -> &[Cow<'_, str>] {
        (*self).get_fields()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Default)]
    struct TestOrderBy<'a> {
        fields: Vec<Cow<'a, str>>,
    }

    impl<'a> OrderBy for TestOrderBy<'a> {
        fn unique_fields(&self) -> &[&[&str]] {
            &[&["id"], &["first_name", "second_name"], &["x", "y", "z"]]
        }

        fn all_fields(&self) -> &[&str] {
            &["id", "first_name", "second_name", "x", "y", "z"]
        }
        fn get_fields(&self) -> &[Cow<'a, str>] {
            &self.fields
        }
    }

    impl<'a> TestOrderBy<'a> {
        fn build<I, S>(fields: I) -> Result<Self, Box<dyn Error + 'static>>
        where
            I: IntoIterator<Item = S> + Clone,
            S: AsRef<str> + Into<Cow<'a, str>>, // 确保每个元素可以转换为 Cow<'a, str>
        {
            let order_by = Self::default();
            validate_order_by(fields.clone(), order_by.all_fields(), order_by.unique_fields())?;

            Ok(Self {
                fields: fields.into_iter().map(Into::into).collect(),
            })
        }
    }

    #[test]
    pub fn test_order_by() -> Result<(), Box<dyn Error + 'static>> {
        let fields = vec!["first_name", "second_name"];
        let order_by = TestOrderBy::build(fields)?;

        assert_eq!(
            order_by
                .get_fields()
                .iter()
                .map(|cow| cow.as_ref())
                .collect::<Vec<_>>(),
            ["first_name", "second_name"]
        );

        Ok(())
    }
}
