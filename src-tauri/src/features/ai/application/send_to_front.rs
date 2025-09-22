use ollama_rs::generation::completion::GenerationResponse;
use tauri::{Emitter, Window};
use tokio_stream::Stream;
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;

pub async fn stream_response_to_frontend<S, E>(
    window: Window,
    stream: &mut S,
    cancellation_token: CancellationToken,
) -> Result<(), String>
where
    S: Stream<Item = Result<Vec<GenerationResponse>, E>> + Unpin,
    E: std::error::Error + Send + Sync + 'static,
{
    while let Some(res) = stream.next().await {
        if cancellation_token.is_cancelled() {
            println!("Stream cancelled");
            break;
        }

        match res {
            Ok(response_chunks) => {
                for response_chunk in response_chunks {
                    // Check for thinking content and emit if it exists
                    if let Some(thinking_text) = response_chunk.thinking {
                        let _ = window.emit("ollama-chat-thinking", thinking_text);
                    }

                    // Check for response content and emit if it exists
                    if !response_chunk.response.is_empty() {
                        let _ = window.emit("ollama-chat-part", response_chunk.response);
                    }

                    // Check if the stream is finished
                    if response_chunk.done {
                        let _ = window.emit("ollama-chat-end", {});
                        return Ok(());
                    }
                }
            }
            Err(e) => {
                eprintln!("Stream error: {:?}", e);
                let _ = window.emit("ollama-chat-end", {});
                return Err(e.to_string());
            }
        }
    }
    let _ = window.emit("ollama-chat-end", {});
    Ok(())
}
