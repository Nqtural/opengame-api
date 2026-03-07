use anyhow::Result;
use async_trait::async_trait;
use types::User;

pub mod postgres;
pub mod types;

pub enum NewUserStatus {
    Success,
    AlreadyExists,
}

#[async_trait]
pub trait Storage: Send + Sync {
    async fn new_user(&self, user: User) -> Result<NewUserStatus>;
}
