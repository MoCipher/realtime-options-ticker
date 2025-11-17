// This file handles WebSocket connections for real-time updates of options data.
// It exports functions like start_websocket and handle_message.

use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::connect_async;
use futures_util::{SinkExt, StreamExt};
use std::error::Error;

pub async fn start_websocket(url: &str) -> Result<(), Box<dyn Error>> {
    let (ws_stream, _) = connect_async(url).await?;
    let (mut write, mut read) = ws_stream.split();

    // Example of sending a message to the WebSocket server
    write.send(Message::Text("Hello, WebSocket!".into())).await?;

    // Listening for messages from the WebSocket server
    while let Some(message) = read.next().await {
        match message? {
            Message::Text(text) => handle_message(text).await,
            Message::Close(_) => break,
            _ => (),
        }
    }

    Ok(())
}

async fn handle_message(message: String) {
    // Process the incoming message (e.g., parse options data)
    println!("Received message: {}", message);
}