use async_trait::async_trait;
use merino_lib::{features::notes::domain::{errors::NoteError, note::Note, repository::NoteRepository}};

#[cfg(test)]
#[derive(Clone)]
pub struct MockNoteRepository {
    pub notes: Vec<Note>,
    pub should_error: bool,
}

#[cfg(test)]
#[async_trait]
impl NoteRepository for MockNoteRepository {
    async fn list_notes(&self, _space_name: &str) -> Result<Vec<Note>, NoteError> {
        if self.should_error {
            return Err(NoteError::NotFound("Mocked error: Space not found".to_string()));
        }
        Ok(self.notes.clone())
    }

    async fn create_note(&self, _space_name: &str, _note_name: &str) -> Result<Note, NoteError> {}
}