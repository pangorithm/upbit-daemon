use thiserror::Error;

/// Application error types
#[derive(Error, Debug)]
pub enum AppError {
    /// Configuration loading/parsing error
    #[error("configuration error: {0}")]
    Config(String),

    /// Database error (from sqlx)
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),

    /// HTTP request error (from reqwest)
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// WebSocket connection/message error
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    /// JSON serialization/deserialization error
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// File system or general IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Upbit API response error
    #[error("API error: {0}")]
    Api(String),

    /// WebSocket connection already closed
    #[error("WebSocket connection closed")]
    ConnectionClosed,
}
