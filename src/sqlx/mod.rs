use async_trait::async_trait;

mod query;

pub use query::QueryData;

pub trait Repository {
    fn table(&self) -> String;
}

#[async_trait]
pub trait Crud {
    type Model: Send + Sync;
    async fn create(&self, data: &QueryData);
}
