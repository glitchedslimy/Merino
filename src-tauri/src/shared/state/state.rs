// src/shared/state.rs

use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use std::sync::Arc;

pub struct AppState {
    pub current_cancellation_token: Arc<Mutex<Option<CancellationToken>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_cancellation_token: Arc::new(Mutex::new(None)),
        }
    }
}