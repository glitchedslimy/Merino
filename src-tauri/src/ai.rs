use ollama_rs::generation::chat::{ChatMessageResponseStream};
use tauri::{Emitter, Window};
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;


pub async fn stream_response_to_frontend(
	window: Window,
	mut stream: ChatMessageResponseStream,
	token: CancellationToken,
) -> Result<(), String> {
	loop {
		tokio::select! {
			_ = token.cancelled() => {
				println!("Stream stopped by user request via CancellationToken!");
				let _ = window.emit("ollama-chat-cancel", {}); // <-- Emit a dedicated cancellation event
				return Ok(());
			}
			next_chunk = stream.next() => {
				match next_chunk {
					Some(Ok(chat_response)) => {
						let _ = window.emit("ollama-chat-chunk", &chat_response);
					},
					Some(Err(e)) => {
						eprintln!("Stream error: {:?}", e);
						let _ = window.emit("ollama-chat-end", {});
						return Err(format!("Stream error: {:?}", e));
					},
					None => {
						println!("Stream ended naturally.");
						let _ = window.emit("ollama-chat-end", {});
						return Ok(());
					}
				}
			}
		}
	}
}
