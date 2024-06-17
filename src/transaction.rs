use async_trait::async_trait;

#[async_trait]
pub trait DbTransaction: Send + Sync {
    async fn commit(self) -> anyhow::Result<()>;
    async fn rollback(self) -> anyhow::Result<()>;
}