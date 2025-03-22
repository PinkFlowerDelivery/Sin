use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errors {
    #[error("Input-Output error: {0}")]
    StdIO(#[from] std::io::Error),

    #[error("Json error {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Invalid header: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),
}
