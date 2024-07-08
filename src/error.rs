use thiserror::Error;

pub type MiasChannelResult<T> = Result<T, MiasChannelError>;

#[derive(Error, Debug)]
pub enum MiasChannelError {
    #[error("Failed to send message: {0}")]
    SendError(String),
    #[error("Failed to receive message: {0}")]
    RecvError(String),
}
