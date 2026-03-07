//use anyhow::Result;
use async_trait::async_trait;

pub mod postgres;

#[async_trait]
pub trait Storage: Send + Sync {
}
