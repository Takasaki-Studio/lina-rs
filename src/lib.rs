#[cfg(feature = "sqlx")]
pub mod sqlx;

pub mod macros {
    pub use lina_rs_macros::{repo, repo_impl};
}
