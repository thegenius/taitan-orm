use sqlx::Database;
use crate::brave_new::location::Location;

pub trait Unique<DB: Database> : Location <DB> { }