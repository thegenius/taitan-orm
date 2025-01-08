use std::fmt::{Debug, Display};
use serde::{Deserialize, Serialize};



//           | arguments | selection             | selected    |
//   None    |  ignore   | ignore                |  ignore     |
//   Null    |  null     | need read             |  read value |
//   Some(T) | set value | not read, set value   |  set value  |
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub enum Optional<T> {
    None,    // 作为入参时表示不传递到数据库层，作为selection时表示
    Null,    // 传递到数据库，值为null，作为selection时表示直接设置为null
    Some(T), // 传递到数据库，值为具体值
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

impl<T> Display for Optional<T> where T: Debug {
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

    pub fn unwrap_or_default(self) -> T where T: Default {
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
