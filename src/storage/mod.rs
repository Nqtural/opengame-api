use anyhow::Result;
use async_trait::async_trait;
use types::{LoginRequest, User};
use uuid::Uuid;

pub mod postgres;
pub mod types;

pub enum GetUserStatus {
    Success(User),
    InvalidCredentials,
    NotFound,
}

pub enum GetCurrentUserStatus {
    Success(User),
    InvalidCredentials,
}

pub enum DeleteAllSessionsStatus {
    Success,
    InvalidCredentials,
}

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
    async fn get_user(&self, username: &str) -> Result<GetUserStatus>;
    async fn get_current_user(&self, bearer: Uuid) -> Result<GetCurrentUserStatus>;
    async fn delete_all_sessions(&self, bearer: Uuid) -> Result<DeleteAllSessionsStatus>;
    async fn delete_session(&self, bearer: Uuid) -> Result<DeleteSessionStatus>;
    async fn new_session(&self, credentials: LoginRequest) -> Result<NewSessionStatus>;
    async fn new_user(&self, user: User) -> Result<NewUserStatus>;
    async fn validate_bearer(&self, bearer: Uuid) -> bool;
}
