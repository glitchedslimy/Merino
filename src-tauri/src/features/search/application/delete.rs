use crate::features::search::{
    domain::error::SearchError, infrastructure::search_repository::TantivySearchRepository,
};
use tantivy::IndexWriter;

pub fn delete_document_use_case(
    repo: &TantivySearchRepository,
    index_writer: &mut IndexWriter,
    route: &str,
) -> Result<(), SearchError> {
    repo.delete_document(index_writer, route.trim())
}
