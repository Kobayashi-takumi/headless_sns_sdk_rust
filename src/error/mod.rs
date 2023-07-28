use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuilderError {
    #[error("{0} must be set.")]
    Required(String),
}
