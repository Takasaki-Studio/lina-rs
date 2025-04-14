use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};

pub trait Repository {
    fn table(&self) -> String;
}

#[async_trait]
pub trait Crud {
    type Model: Serialize + DeserializeOwned + Send + Sync;
    async fn create(&self, model: &Self::Model);
}
