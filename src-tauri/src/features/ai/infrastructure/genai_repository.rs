use async_trait::async_trait;
use log::info;
use ollama_rs::{
    generation::{chat::ChatMessage, completion::request::GenerationRequest},
    Ollama,
};
use scraper::{ElementRef, Html, Selector};
use tauri::{AppHandle, Emitter, State, Window};
use tokio_util::sync::CancellationToken;

use crate::{
    features::ai::{
        application::send_to_front::stream_response_to_frontend,
        domain::{
            ai::{ModelResponse, OllamaWebResponse},
            repository::AIRepository,
        },
    },
    shared::state::state::AppState,
};

#[derive(Clone)]
pub struct GenAIRepository {
    app_handle: AppHandle,
}

impl GenAIRepository {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }
}

#[async_trait]
impl AIRepository for GenAIRepository {
    async fn get_ai_models(&self) -> Result<Vec<ModelResponse>, String> {
        let ollama = Ollama::default();
        let local_models = ollama
            .list_local_models()
            .await
            .map_err(|e| e.to_string())?;

        let mut handles = Vec::new();
        for model in local_models {
            let ollama_clone = ollama.clone();
            let model_name = model.name;
            let handle = tokio::spawn(async move {
                let model_info = ollama_clone
                    .show_model_info(model_name.clone())
                    .await
                    .map_err(|e| e.to_string())?;

                Ok(ModelResponse {
                    name: model_name,
                    capabilities: Some(model_info.capabilities),
                })
            });
            handles.push(handle);
        }

        let mut all_model_info = Vec::new();
        for handle in handles {
            let result: Result<ModelResponse, String> = handle.await.map_err(|e| e.to_string())?;
            all_model_info.push(result?);
        }

        Ok(all_model_info)
    }

    async fn chat_with_ai(
        &self,
        window: Window,
        prompt: Vec<ChatMessage>,
        model_name: String,
        use_thinking: bool,
        use_tools: bool,
        app_state: State<'_, AppState>,
    ) -> Result<(), String> {
        let ollama = Ollama::default();
        let full_prompt = prompt
            .iter()
            .map(|m| m.content.clone())
            .collect::<Vec<String>>()
            .join("\n");
        let mut request = GenerationRequest::new(model_name.clone(), full_prompt);

        if use_thinking {
            request = request.think(use_thinking);
        }
        // This is a placeholder for your tool implementation
        if use_tools {
            // Your tool logic here
        }

        let new_token = CancellationToken::new();
        let new_token_for_task = new_token.clone();

        {
            let mut token_guard = app_state.current_cancellation_token.lock().await;
            if let Some(token) = token_guard.take() {
                token.cancel();
            }
            *token_guard = Some(new_token);
        }

        tokio::spawn(async move {
            let res = ollama.generate_stream(request).await;

            match res {
                Ok(mut stream) => {
                    let _ =
                        stream_response_to_frontend(window, &mut stream, new_token_for_task).await;
                }
                Err(e) => {
                    eprintln!("Ollama API error: {:?}", e);
                    let _ = window.emit("ollama-chat-end", {});
                }
            }
        });

        Ok(())
    }

    async fn cancel_stream(&self, state: State<'_, AppState>) -> Result<(), String> {
        let mut token_guard = state.current_cancellation_token.lock().await;
        if let Some(token) = token_guard.take() {
            token.cancel();
        }
        Ok(())
    }

    async fn check_ollama_status(&self) -> Result<bool, String> {
        let ollama_client: Ollama = Ollama::default();

        let models = ollama_client.list_local_models().await;

        match models {
            Ok(models_list) => Ok(!models_list.is_empty()),
            Err(_) => Ok(false),
        }
    }

    async fn create_ollama_model(&self, model_name: String) -> Result<(), String> {
        let ollama_client = Ollama::default();
        let model = ollama_client.pull_model(model_name, false).await;

        match model {
            Ok(_model_response) => Ok(()),
            Err(_) => Err("Error creating Ollama Model".to_string()),
        }
    }

    async fn get_web_models(&self) -> Result<Vec<OllamaWebResponse>, String> {
        // The URL for the Ollama library page.
        let url = "https://ollama.com/library";

        // Make an asynchronous HTTP GET request to the URL.
        let response = reqwest::get(url)
            .await
            .map_err(|e| format!("Failed to fetch URL: {}", e))?
            .text()
            .await
            .map_err(|e| format!("Failed to get response text: {}", e))?;

        // Parse the HTML content.
        let document = Html::parse_document(&response);

        // Use a CSS selector to find all model entries.
        let model_selector = Selector::parse("a[href^='/library/']")
            .map_err(|_| "Failed to parse model selector".to_string())?;

        let mut models_data = Vec::new();

        // Iterate over each model entry found.
        for element in document.select(&model_selector) {
            // Selectors for individual fields within each model entry.
            let name_selector = Selector::parse("h2").unwrap();
            let description_selector = Selector::parse("p.max-w-lg.text-neutral-800").unwrap();
            let capabilities_selector = Selector::parse("span[x-test-capability]").unwrap();
            let sizes_selector = Selector::parse("span[x-test-size]").unwrap();
            let pulls_selector = Selector::parse("span[x-test-pull-count]").unwrap();
            let date_selector = Selector::parse("span[x-test-updated]").unwrap();

            // Extract data for each field, handling cases where the element might not be found.
            let model_name = element
                .select(&name_selector)
                .next()
                .map_or("".to_string(), |e| {
                    e.text().collect::<Vec<_>>().join("").trim().to_string()
                });

            let description = element
                .select(&description_selector)
                .next()
                .map_or("".to_string(), |e| {
                    e.text().collect::<Vec<_>>().join("").trim().to_string()
                });

            let sizes: Vec<String> = element
                .select(&sizes_selector)
                .map(|e| e.text().collect::<Vec<_>>().join("").trim().to_string())
                .collect();

            let capabilities: Vec<String> = element
                .select(&capabilities_selector)
                .map(|e| e.text().collect::<Vec<_>>().join("").trim().to_string())
                .collect();

            let pulls = element
                .select(&pulls_selector)
                .next()
                .map_or("".to_string(), |e| {
                    e.text().collect::<Vec<_>>().join("").trim().to_string()
                });

            let date = element
                .select(&date_selector)
                .next()
                .map_or("".to_string(), |e| {
                    e.text().collect::<Vec<_>>().join("").trim().to_string()
                });

            // Create the struct instance and push it to the vector.
            models_data.push(OllamaWebResponse {
                model_name,
                description,
                sizes,
                capabilities,
                pulls,
                date,
            });
        }

        Ok(models_data)
    }

    async fn delete_ollama_model(&self, model_name: String) -> Result<(), String> {
         let ollama_client = Ollama::default();
        let model = ollama_client.delete_model(model_name).await;

        match model {
            Ok(_model_response) => Ok(()),
            Err(_) => Err("Error deleting Ollama Model".to_string()),
        }
    }
}
