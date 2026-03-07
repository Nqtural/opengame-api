use anyhow::Result;
use async_trait::async_trait;
use types::{LoginRequest, User};
use uuid::Uuid;

pub mod postgres;
pub mod types;

pub enum DeleteSessionStatus {
    Success,
    InvalidCredentials,
}

pub enum NewSessionStatus {
    Success(Uuid),
    InvalidCredentials,
}

pub enum NewUserStatus {
    Success,
    AlreadyExists,
}

#[async_trait]
pub trait Storage: Send + Sync {
    async fn delete_session(&self, bearer: Uuid) -> Result<DeleteSessionStatus>;
    async fn new_session(&self, credentials: LoginRequest) -> Result<NewSessionStatus>;
    async fn new_user(&self, user: User) -> Result<NewUserStatus>;
}
