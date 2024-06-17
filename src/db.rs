use async_trait::async_trait;

use crate::transaction::DbTransaction;

#[async_trait]
pub trait Db {
    type Transaction<'a>: DbTransaction;

    async fn begin<'a>(&self) -> anyhow::Result<Self::Transaction<'a>>;
}