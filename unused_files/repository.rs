use crate::error::ErrDB;
use async_trait::async_trait;


#[async_trait]
pub trait DBRepository<T>: Send + Sync {
    async fn insert_data(&mut self, data: &T) -> Result<(), ErrDB>;
    async fn remove_data(&mut self, data: &T) -> Result<(), ErrDB>;
    async fn list(&self) -> Result<Vec<T>, ErrDB>;
    async fn is_exist(&self, data: &T) -> Result<bool, ErrDB>;
}
