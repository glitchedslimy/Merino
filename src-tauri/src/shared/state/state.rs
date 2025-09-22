use std::sync::Arc;
use tantivy::IndexWriter;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

use crate::features::ai::infrastructure::genai_repository::GenAIRepository;
use crate::features::folders::infrastructure::filesystem_repository::FileSystemFolderRepository;
use crate::features::notes::infrastructure::filesystem_repository::FileSystemNoteRepository;
use crate::features::search::infrastructure::search_repository::TantivySearchRepository;
use crate::features::settings::infrastructure::settings_repository::FileSystemSettingsRepository;
use crate::features::space::infrastructure::filesystem_repo::FileSystemSpaceRepository;

pub struct AppState {
    pub filesystem_repo: Arc<Mutex<FileSystemNoteRepository>>,
    pub spaces_repo: Arc<Mutex<FileSystemSpaceRepository>>,
    pub folders_repo: Arc<Mutex<FileSystemFolderRepository>>,
    pub search_repo: Arc<Mutex<TantivySearchRepository>>,
    pub ai_repo: Arc<Mutex<GenAIRepository>>,
    pub settings_repo: Arc<Mutex<FileSystemSettingsRepository>>,
    pub current_cancellation_token: Arc<Mutex<Option<CancellationToken>>>,
    pub index_writer: Arc<Mutex<IndexWriter>>,
}

impl AppState {
    pub fn new(
        notes_repo: FileSystemNoteRepository,
        spaces_repo: FileSystemSpaceRepository,
        folders_repo: FileSystemFolderRepository,
        search_repo: TantivySearchRepository,
        ai_repo: GenAIRepository,
        settings_repo: FileSystemSettingsRepository,
        index_writer: IndexWriter,
    ) -> Self {
        Self {
            filesystem_repo: Arc::new(Mutex::new(notes_repo)),
            spaces_repo: Arc::new(Mutex::new(spaces_repo)),
            folders_repo: Arc::new(Mutex::new(folders_repo)),
            search_repo: Arc::new(Mutex::new(search_repo)),
            ai_repo: Arc::new(Mutex::new(ai_repo)),
            settings_repo: Arc::new(Mutex::new(settings_repo)),
            index_writer: Arc::new(Mutex::new(index_writer)),
            current_cancellation_token: Arc::new(Mutex::new(None)),
        }
    }
}
