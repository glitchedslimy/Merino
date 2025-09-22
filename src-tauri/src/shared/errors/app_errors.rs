use thiserror::Error;

/// # AppError
/// _Controls the general and specific errors in the application._
///
/// The error types that can be used are:
/// * `Io`: Io related errors
/// * `Note`: Note Specific errors
/// * `Internal`: Something unexpected but app related happened
/// * `Unknown`: We don't know what happened or where it comes from
#[derive(Debug, Error)]
pub enum AppError {
    /// # Tauri IO Errors
    /// _Expects a [tauri::Error]_.
    #[error("Tauri IO error: {0}")]
    TauriIo(#[from] tauri::Error),

    /// # IO Errors
    /// _Expecs a [std::io::Error]
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    /// # Internal
    /// App Error related, but unknown of why it happend.
    #[error("An unexpected error ocurred: {0}")]
    Internal(String),

    /// # Unknown
    /// We don't know where this error comes from neither why it happend.
    #[error("Something happened.")]
    Unknown,
}

/// # From Implementation
/// This is just to comply with some Rust behaviour when it doesn't
/// know the error that comes, we default it to Unknown.
impl From<()> for AppError {
    fn from(_: ()) -> Self {
        AppError::Unknown
    }
}
