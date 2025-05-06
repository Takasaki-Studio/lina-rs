use async_trait::async_trait;
use sqlx::{Acquire, Database};

mod query;

pub use query::QueryData;

pub trait Repository<T> {
    type DB: Database;
    fn new<'a>(connection: T) -> Self
    where
        T: Acquire<'a, Database = Self::DB>;
}

#[async_trait]
pub trait Crud {
    type Model: Send + Sync;
    async fn create(&self, data: &QueryData);
}
