use std::borrow::Cow;

#[derive(Debug, PartialEq, Clone)]
pub struct FieldName {
    pub name: Cow<'static, str>,
    pub is_null: bool,
    database_field_alias: Option<Cow<'static, str>>,
}

impl FieldName {
    pub fn new(name: Cow<'static, str>, is_null: bool) -> Self {
        Self { name, is_null, database_field_alias: None }
    }

    pub fn from_str(name: &'static str, is_null: bool) -> Self {
        Self {
            name: Cow::Borrowed(name),
            is_null,
            database_field_alias: None,
        }
    }

    pub fn with_alias(name: Cow<'static, str>, is_null: bool, database_field_alias: Option<&'static str>) -> Self {
        match database_field_alias {
            Some(database_field_alias) => {
                Self {
                    name,
                    is_null,
                    database_field_alias: Some(Cow::Borrowed(database_field_alias)),
                }
            }
            None => {
                Self {
                    name,
                    is_null,
                    database_field_alias: None
                }
            }
        }
    }

    pub fn database_field_name(&self) -> &str {
        match &self.database_field_alias {
            Some(database_field_name) => database_field_name,
            None => &self.name,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::field::FieldName;

    #[test]
    pub fn test_field_name() {
        let field_name = FieldName::from_str("foo", true);
    }
}