use sqlx::{Acquire, Database};

mod query;

pub use query::QueryData;

pub trait Repository<T> {
    type DB: Database;

    fn new<'a>(connection: T) -> Self
    where
        T: Acquire<'a, Database = Self::DB>;
}
