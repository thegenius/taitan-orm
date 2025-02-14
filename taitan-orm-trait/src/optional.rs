use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display};

//           | arguments | selection             | selected    |
//   None    |  ignore   | ignore                |  ignore     |
//   Null    |  null     | need read             |  read value |
//   Some(T) | set value | not read, set value   |  set value  |
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Optional<T> {
    None,    // 作为入参时表示不传递到数据库层，作为selection时表示
    Null,    // 传递到数据库，值为null，作为selection时表示直接设置为null
    Some(T), // 传递到数据库，值为具体值
}

impl<T: Serialize> Serialize for Optional<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Optional::None => serializer.serialize_str(""),
            Optional::Null => serializer.serialize_none(),
            Optional::Some(value) => value.serialize(serializer),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Optional<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // 自定义 Visitor
        struct OptionalVisitor<T>(std::marker::PhantomData<T>);

        impl<'de, T: Deserialize<'de>> Visitor<'de> for OptionalVisitor<T> {
            type Value = Optional<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string, null, or a value")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.is_empty() {
                    Ok(Optional::None)
                } else {
                    Err(serde::de::Error::custom(
                        "expected an empty string for None",
                    ))
                }
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Optional::Null)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                T::deserialize(deserializer).map(Optional::Some)
            }
        }

        deserializer.deserialize_any(OptionalVisitor(std::marker::PhantomData))
    }
}

impl<T> PartialEq<Option<T>> for Optional<T> {
    fn eq(&self, other: &Option<T>) -> bool {
        match self {
            Optional::None => matches!(other, None),
            Optional::Some(s) => matches!(other, Some(s)),
            _ => false,
        }
    }
}

impl<T> Display for Optional<T>
where
    T: Debug,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Optional::None => write!(fmt, ""),
            Optional::Null => write!(fmt, ""),
            Optional::Some(s) => write!(fmt, "{:?}", s),
        }
    }
}

impl<T> Default for Optional<T> {
    fn default() -> Self {
        Self::None
    }
}

impl<T> From<Option<T>> for Optional<T> {
    fn from(option: Option<T>) -> Self {
        match option {
            Some(value) => Optional::Some(value),
            None => Optional::None,
        }
    }
}

impl<T> Optional<T> {
    pub fn unwrap(self) -> T {
        match self {
            Optional::Some(value) => value,
            _ => panic!("called `Optional::unwrap()` on a `None`"),
        }
    }

    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Optional::Some(value) => value,
            _ => default,
        }
    }

    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            Optional::Some(value) => value,
            _ => Default::default(),
        }
    }

    pub fn not_some(&self) -> bool {
        match self {
            Optional::Some(_) => false,
            _ => true,
        }
    }

    pub fn not_none(&self) -> bool {
        match self {
            Optional::None => false,
            _ => true,
        }
    }

    pub fn not_null(&self) -> bool {
        match self {
            Optional::Null => false,
            _ => true,
        }
    }

    pub fn not_selected(&self) -> bool {
        match self {
            Optional::Null => false,
            _ => true,
        }
    }
    pub fn is_selected(&self) -> bool {
        match self {
            Optional::Null => true,
            _ => false,
        }
    }

    pub fn is_some(&self) -> bool {
        match self {
            Optional::Some(_) => true,
            _ => false,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            Optional::None => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            Optional::Null => true,
            _ => false,
        }
    }
}
