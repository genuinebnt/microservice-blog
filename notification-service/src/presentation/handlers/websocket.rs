use std::sync::Arc;

use axum::{
    extract::{
        Path, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use uuid::Uuid;

use crate::presentation::state::AppState;

pub async fn ws_notifications(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, user_id))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>, user_id: Uuid) {
    let mut rx = state.tx.subscribe();

    tracing::info!("WebSocket connected for user: {}", user_id);

    loop {
        tokio::select! {
            Ok(msg) = rx.recv() => {
                if msg.user_id == user_id {
                    let json = serde_json::to_string(&msg).unwrap();
                    if socket.send(json.into()).await.is_err() {
                        break;
                    }
                }
            }

            msg = socket.recv() => {
               match msg {
                Some(Ok(Message::Close(_))) | None => break,
                Some(Ok(Message::Ping(msg))) => {
                    if socket.send(Message::Pong(msg)).await.is_err() {
                        break;
                    }
                }
                _ => {}
               }
            }
        }
    }
}
