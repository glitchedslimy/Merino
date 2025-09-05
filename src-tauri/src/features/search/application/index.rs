use crate::features::search::{
    domain::{error::SearchError, search::Searchable},
    infrastructure::search_repository::TantivySearchRepository,
};
use tantivy::IndexWriter;

pub fn index_document_use_case<T: Searchable>(
    repo: &TantivySearchRepository,
    index_writer: &mut IndexWriter,
    document: &T,
    space_name: &str,
) -> Result<(), SearchError> {
    repo.index_document(index_writer, document, space_name)
}