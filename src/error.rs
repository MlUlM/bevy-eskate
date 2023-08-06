use thiserror::Error;

pub type GameResult<T = ()> = Result<T, GameError>;


#[derive(Debug, Error)]
pub enum GameError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}