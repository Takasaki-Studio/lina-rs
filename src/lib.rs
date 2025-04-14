#[cfg(feature = "error")]
pub mod error;

#[cfg(feature = "sqlx")]
pub mod sqlx;

pub mod macros {
    pub use lina_rs_macros::repo;
}

pub mod prelude {
    pub use async_trait::async_trait;
}
