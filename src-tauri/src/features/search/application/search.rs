use crate::features::search::{
    domain::error::SearchError, infrastructure::search_repository::TantivySearchRepository,
};

pub fn search_documents_use_case(
    repo: &TantivySearchRepository,
    query: &str,
) -> Result<Vec<String>, SearchError> {
    repo.search_documents(query)
}
