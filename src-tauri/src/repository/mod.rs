use async_trait::async_trait;
use anyhow::Result;

/// 通用 Repository Trait
#[async_trait]
pub trait Repository<T, ID> {
    async fn find_by_id(&self, id: ID) -> Result<Option<T>>;
    async fn find_all(&self) -> Result<Vec<T>>;
    async fn insert(&self, entity: &T) -> Result<()>;
    async fn update(&self, entity: &T) -> Result<()>;
    async fn delete(&self, id: ID) -> Result<()>;
}

pub mod user_repo;
pub mod strategy_repo;

pub use user_repo::UserRepository;
pub use strategy_repo::StrategyRepository;
