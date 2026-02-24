use std::sync::Arc;

use common::{outbox::OutBoxEvent, pubsub::PubSubSubscriber};
use uuid::Uuid;

use crate::{
    domain::entities::notification::Notification,
    presentation::state::{AppState, NotificationEvent},
};

async fn process_event(state: &Arc<AppState>, event: OutBoxEvent) {
    let (user_id, kind, title, message) = match event.event_type.as_str() {
        "post_created" => {
            let author_id = event.payload["author_id"]
                .as_str()
                .and_then(|s| s.parse().ok());
            let post_title = event.payload["title"]
                .as_str()
                .map(|s| s.to_string())
                .unwrap_or("Untitled".to_string());

            match author_id {
                Some(uid) => (
                    uid,
                    "post_created".to_string(),
                    "New Post Published".to_string(),
                    format!("Your post '{}' has been published!", post_title),
                ),
                None => return,
            }
        }
        "user_registered" => {
            let user_id = event.payload["user_id"]
                .as_str()
                .and_then(|s| s.parse().ok());
            let username = event.payload["username"]
                .as_str()
                .map(|s| s.to_string())
                .unwrap_or("User".to_string());

            match user_id {
                Some(uid) => (
                    uid,
                    "Welcome".to_string(),
                    "Welcome to Our Blog".to_string(),
                    format!(
                        "Hello {}, welcome to our blog! Start sharing your thoughts",
                        username
                    ),
                ),
                None => return,
            }
        }
        _ => return,
    };

    let notification = Notification {
        id: Uuid::new_v4(),
        user_id,
        kind: kind.clone(),
        title: title.clone(),
        message: message.clone(),
        is_read: false,
        created_at: chrono::Utc::now().into(),
    };

    match state
        .repos
        .notifications
        .create_notification(notification)
        .await
    {
        Ok(_) => {
            tracing::info!("Notification created for user: {}", user_id);
        }
        Err(e) => {
            tracing::error!("Failed to create notification: {}", e);
        }
    }

    let _ = state.tx.send(NotificationEvent {
        user_id,
        kind,
        title,
        message,
    });
}

pub fn spawn_subscriber(state: Arc<AppState>, subscriber: PubSubSubscriber) {
    tokio::spawn(async move {
        tracing::info!("Pub/Sub subscriber started");

        if let Err(e) = subscriber
            .listen(move |msg| {
                let state = state.clone();
                async move {
                    let data = String::from_utf8_lossy(&msg.message.data);
                    match serde_json::from_str::<OutBoxEvent>(&data) {
                        Ok(event) => process_event(&state, event).await,
                        Err(e) => tracing::error!("Failed to parse event: {}", e),
                    }
                    let _ = msg.ack().await;
                }
            })
            .await
        {
            tracing::error!("Failed to start subscriber: {}", e);
        }
    });
}
