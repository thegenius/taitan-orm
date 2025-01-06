use crate::selection::Selection;
use crate::NotImplementError;
use sqlx::types::Uuid;
use sqlx::{ColumnIndex, Database, Decode, Type};
use std::fmt::Debug;
use time::PrimitiveDateTime;

pub trait SelectedEntity<DB: Database>: Debug + Default + Selection {
    // type Selection: Selection;

    // fn from_row(selection: &Self::Selection, row: DB::Row) -> Result<Self, sqlx::Error>
    // where
    //     Self: Sized;

    fn select_from_row(selection: &Self, row: DB::Row) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        todo!()
    }

    fn from_row_full(row: DB::Row) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        Err(sqlx::Error::Decode(
            NotImplementError("".to_string()).into(),
        ))
    }
}

// pub trait SelectedEntityNew: Debug + Default {
//     type Selection: Selection;
//     fn from_row<DB: Database>(
//         selection: &Self::Selection,
//         row: DB::Row,
//     ) -> Result<Self, sqlx::Error>
//     where
//         Self: Sized,
//         for<'a> PrimitiveDateTime: Type<DB> + Decode<'a, DB>,
//         for<'a> i32: Type<DB> + Decode<'a, DB>,
//         for<'a> String: Type<DB> + Decode<'a, DB>,
//         for<'a> Uuid: Type<DB> + Decode<'a, DB>,
//         for<'a> u64: Type<DB> + Decode<'a, DB>,
//         for<'a> &'a str: ColumnIndex<DB::Row>,
//         usize: ColumnIndex<DB::Row>;
//
//     fn from_row_full<DB: Database>(row: DB::Row) -> Result<Self, sqlx::Error>
//     where
//         Self: Sized,
//     {
//         Err(sqlx::Error::Decode(
//             NotImplementError("".to_string()).into(),
//         ))
//     }
// }
