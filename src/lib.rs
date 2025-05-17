#[cfg(feature = "sqlx")]
pub mod sqlx;

#[cfg(feature = "sqlx")]
pub mod macros {
    pub use lina_rs_macros::{repo, repo_impl};
}
