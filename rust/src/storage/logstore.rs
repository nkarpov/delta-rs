//! LogStore trait for all Delta Log related access

use async_trait::async_trait;

#[derive(Debug)]
#[allow(missing_docs)]
/// Errors thrown by LogStore
pub enum LogStoreError {
    WriteError,
    ReadError,
    ListFromError,
}

/// Result
pub type LogStoreResult<T, E = LogStoreError> = std::result::Result<T, E>;

#[async_trait]
/// Log related access
pub trait LogStore {
    /// write
    async fn write(&self) -> LogStoreResult<()> {
        todo!()
    }

    /// read
    async fn read(&self) -> LogStoreResult<()> {
        todo!()
    }

    /// list_from
    async fn list_from(&self) -> LogStoreResult<()> {
        todo!()
    }
}