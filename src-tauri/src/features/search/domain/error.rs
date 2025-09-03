use serde::Serialize;
use tantivy::query::QueryParserError;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum SearchError {
    #[error("Tantivy error: {0}")]
    TantivyError(String),
    #[error("I/O Error: {0}")]
    IoError(String),
    #[error("Query error: {0}")]
    Query(String)
}

impl From<tantivy::TantivyError> for SearchError {
    fn from(err: tantivy::TantivyError) -> Self {
        SearchError::TantivyError(err.to_string())
    }
}

impl From<std::io::Error> for SearchError {
    fn from(err: std::io::Error) -> Self {
        SearchError::IoError(err.to_string())
    }
}

impl From<QueryParserError> for SearchError {
    fn from(err: QueryParserError) -> Self {
        SearchError::Query(err.to_string())
    }
}