use thiserror::Error;

#[derive(Error, Debug)]
pub enum WorldError {
    #[error("RPC Error: {0}")]
    RpcError(String),
    #[error("Serialization Error")]
    SerializationError,
}