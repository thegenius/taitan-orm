use serde::de::Deserialize;
use serde::Serialize;

pub type JoinedFields = (JoinedField, JoinedField);
#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct JoinedField {
    pub(crate) table_name: String,
    pub(crate) field_name: String,
}

impl<'de> Deserialize<'de> for JoinedField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let content = String::deserialize(deserializer)?;
        let pair: Vec<&str> = content.split('.').collect();
        if pair.len() != 2 {
            return Err(serde::de::Error::custom(
                "join field must have table name, and seperate by '.' ",
            ));
        }
        Ok(Self {
            table_name: pair.first().unwrap().to_string(),
            field_name: pair.last().unwrap().to_string(),
        })
    }
}
