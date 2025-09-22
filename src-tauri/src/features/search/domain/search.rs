use std::path::PathBuf;

use crate::features::notes::domain::note::Note;

pub trait Searchable {
    fn get_unique_id(&self, space_name: &str) -> String;
    fn get_search_name(&self) -> String;
    fn get_search_content(&self) -> Option<&str>;
    fn get_search_folder(&self) -> Option<&str>;
}

impl Searchable for Note {
    fn get_unique_id(&self, space_name: &str) -> String {
        let mut path = PathBuf::new();
        path.push(space_name);

        if let Some(folder) = &self.folder {
            path.push(folder);
        }

        // Crucially, add the .md extension here
        path.push(format!("{}.md", &self.name));

        path.to_str().unwrap().replace("\\", "/")
    }

    fn get_search_name(&self) -> String {
        self.name.clone()
    }

    fn get_search_content(&self) -> Option<&str> {
        self.content.as_deref()
    }

    fn get_search_folder(&self) -> Option<&str> {
        self.folder.as_deref()
    }
}
