```rust
// 1. must derive Schema
#[derive(Debug, Clone, Schema)]

// 2. optional table attribute,
//    used to specify the table name. 
//    if not exists, parser will use struct name snake case as the table name
#[table(user)]

// 3. required primary attribute
//    used to specify the primary keys
//    macro expander will generate following struct for you:
//    
//    pub struct UserPrimary {
//        id: i32
//    }
#[primary(id)]

// 4. optional unique attribute,
//    uk_xxx is the unique index name, recommend begin with uk_
//    (name) is the fields of this unique index
//    multi unique(...) is allowed
//    macro expander will generate following struct for you:
//    
//    pub struct UserNameUnique {
//        name: String
//    }
#[unique(uk_name=(name))]

// 5. optional index attributes,
//    idx_xxx is the index name, recommend begin with idx_
//    (age, birthday) is the fields of this index
//    multi index(...) is allowed
//    macro expander will generate following struct for you:
//
//    pub enum UserIndexIdxHello {
//        Age {
//            age: LocationExpr<i32>
//        },
//        AgeBirthday{
//            age: LocationExpr<i32>,
//            birthday: LocationExpr<PrimitiveDateTime>
//        }
//    }
//   
//   The index struct will make you enforce the prefix matching rule
//   age is allowed,
//   age, birthday is allowed
//   birthday is not allowed
#[index(idx_hello=(age, birthday))]

pub struct User {
    id: i32,
    name: String,
    age: Option<i32>,
    birthday: Option<PrimitiveDateTime>,
}
```

Except for the attributes related struct/enum, following struct/enum will be auto generated for you:
```rust

// struct for update 
// XxxMutation includes all fields except primary keys.
// 
// xxxMutation can gracefully express null or skip
// [1] None: skip this field
// [2] Some(None): null
// [3] Some(Some(23)): actual set value
// before 0.1.9, there is a special enum Optional to support null expression
// 0.1.10 remove Optional, use Option<Option<T>> instead.
pub struct UserMutation {
    name: Option<Option<String>>,
    age: Option<Option<i32>>,
    birthday: Option<Option<PrimitiveDateTime>>
}

// struct for express where condition
// you can combine this enum with and/or/not
// for example:
// let location = And::new(
//     UserLocation::Id(Expr {
//         cmp: Cmp::GreaterOrEq,
//         val: Some(1),
//     }),
//     UserLocation::Age(Expr {
//         cmp: Cmp::GreaterOrEq,
//         val: Some(24),
//     }),
// );
pub enum UserLocation {
    Id(Expr<i32>),
    Name(Expr<String>),
    Age(Expr<i32>),
    Birthday(Expr<PrimitiveDateTime>)
}

// struct to select field and recv result from database 
pub struct UserSelected {
    id: Option<Option<i32>>,
    name: Option<Option<String>>,
    age: Option<Option<i32>>,
    birthday: Option<Option<PrimitiveDateTime>>
}
```



