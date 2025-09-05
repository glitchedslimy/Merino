use std::{fs, path::Path};

use log::info;
use tantivy::{
    collector::TopDocs, doc, query::QueryParser, schema::{Schema, Value, STORED, TEXT}, Index, IndexWriter, TantivyDocument
};

use crate::features::search::domain::{error::SearchError, search::Searchable};

#[derive(Clone)]
pub struct TantivySearchRepository {
    index: Index,
    schema: Schema,
}

impl TantivySearchRepository {
    pub fn new(index_path: &Path) -> Result<Self, SearchError> {
        let mut schema_builder = Schema::builder();
        
        // This is the key fix. We need to add the `INDEXED` flag to the "route" field
        // so that we can search on it, which is required for the delete_query to work.
        schema_builder.add_text_field("route", TEXT | STORED);
        schema_builder.add_text_field("name", TEXT | STORED);
        schema_builder.add_text_field("content", TEXT | STORED);
        schema_builder.add_text_field("folder", TEXT | STORED);
        let schema = schema_builder.build();

        let index = if !index_path.exists() {
            fs::create_dir_all(&index_path)?;
            Index::create_in_dir(index_path, schema.clone())?
        } else {
            Index::open_in_dir(index_path)?
        };

        Ok(Self { index, schema })
    }
    
    pub fn get_index_writer(&self) -> Result<IndexWriter, SearchError> {
        Ok(self.index.writer(50_000_000)?)
    }

    pub fn index_document<T: Searchable>(
        &self,
        index_writer: &mut IndexWriter,
        document: &T,
        space_name: &str,
    ) -> Result<(), SearchError> {
        let route_field = self.schema.get_field("route").unwrap();
        let name_field = self.schema.get_field("name").unwrap();
        let content_field = self.schema.get_field("content").unwrap();
        let folder_field = self.schema.get_field("folder").unwrap();

        let new_doc = doc!(
            route_field => document.get_unique_id(space_name),
            name_field => document.get_search_name(),
            content_field => document.get_search_content().unwrap_or_default(),
            folder_field => document.get_search_folder().unwrap_or_default(),
        );

        index_writer.add_document(new_doc)?;
        Ok(())
    }
    
    // This method is fine as it is, as the `commit` is handled by the caller.
    pub fn delete_document(&self, index_writer: &mut IndexWriter, document_route: &str) -> Result<(), SearchError> {
        let route_field = self.schema.get_field("route").unwrap();
        
        // Create a query parser for the "route" field.
        let query_parser = QueryParser::for_index(&self.index, vec![route_field]);
        
        // Create a query that looks for the exact document route.
        let query = query_parser.parse_query(&format!("\"{}\"", document_route))?;
        
        // Use delete_query to delete all documents that match the query.
        // This is more reliable than delete_term because it uses the full query parser logic.
        index_writer.delete_query(Box::new(query))?;
        
        Ok(())
    }

    pub fn search_documents(&self, query: &str) -> Result<Vec<String>, SearchError> {
        let reader = self.index.reader()?;
        let searcher = reader.searcher();

        let route_field = self.schema.get_field("route").unwrap();
        let name_field = self.schema.get_field("name").unwrap();
        let content_field = self.schema.get_field("content").unwrap();
        let folder_field = self.schema.get_field("folder").unwrap();

        let query_parser =
            QueryParser::for_index(&self.index, vec![name_field, content_field, folder_field]);
        let tantivy_query = query_parser.parse_query(query)?;
        let top_docs = searcher.search(&tantivy_query, &TopDocs::with_limit(10))?;

        let mut results = Vec::new();
        for (_score, doc_address) in top_docs {
            let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
            let route = retrieved_doc
                .get_first(route_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            results.push(route);
        }
        Ok(results)
    }
}
