use std::sync::Arc;
use tokio::sync::Mutex;

use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::{
    client::{Client, ClientConfig},
    publisher::Publisher,
    subscriber::ReceivedMessage,
    subscription::{Subscription, SubscriptionConfig},
};

use crate::config::PubSubSettings;
use crate::error::{AppError, Result};

#[derive(Clone)]
pub struct PubSubPublisher {
    publisher: Arc<Mutex<Publisher>>,
}

impl PubSubPublisher {
    pub async fn new(settings: &PubSubSettings) -> Result<Self> {
        if settings.use_emulator {
            unsafe { std::env::set_var("PUBSUB_EMULATOR_HOST", &settings.emulator_host) };
        }

        let config = ClientConfig {
            project_id: Some(settings.project_id.clone()),
            ..Default::default()
        };

        let client = Client::new(config)
            .await
            .map_err(|e| AppError::PubSubError(e.to_string()))?;
        let topic = client.topic(&settings.topic);

        if !topic
            .exists(None)
            .await
            .map_err(|e| AppError::PubSubError(e.to_string()))?
        {
            topic
                .create(None, None)
                .await
                .map_err(|e| AppError::PubSubError(e.to_string()))?;
        }

        let publisher = topic.new_publisher(None);

        Ok(Self {
            publisher: Arc::new(Mutex::new(publisher)),
        })
    }

    pub async fn publish(&self, message: String) -> Result<()> {
        let publisher = self.publisher.lock().await;
        let awaiter = publisher
            .publish(PubsubMessage {
                data: message.into_bytes(),
                ..Default::default()
            })
            .await;
        awaiter
            .get()
            .await
            .map_err(|e| AppError::PubSubError(e.to_string()))?;
        Ok(())
    }
}

pub struct PubSubSubscriber {
    subscription: Subscription,
}

impl PubSubSubscriber {
    pub async fn new(settings: &PubSubSettings) -> Result<Self> {
        if settings.use_emulator {
            unsafe {
                std::env::set_var("PUBSUB_EMULATOR_HOST", &settings.emulator_host);
            }
        }

        let config = ClientConfig {
            project_id: Some(settings.project_id.clone()),
            ..Default::default()
        };
        let client = Client::new(config)
            .await
            .map_err(|e| AppError::PubSubError(e.to_string()))?;

        let subscription_name = settings
            .subscription
            .as_ref()
            .expect("subscription name required for subscriber");
        let subscription = client.subscription(subscription_name);

        if !subscription
            .exists(None)
            .await
            .map_err(|e| AppError::PubSubError(e.to_string()))?
        {
            let topic = client.topic(&settings.topic);
            let sub_config = SubscriptionConfig::default();
            subscription
                .create(topic.fully_qualified_name(), sub_config, None)
                .await
                .map_err(|e| AppError::PubSubError(e.to_string()))?;
        }

        Ok(Self { subscription })
    }

    pub async fn listen<F, Fut>(&self, handler: F) -> Result<()>
    where
        F: Fn(ReceivedMessage) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send,
    {
        loop {
            let messages = self
                .subscription
                .pull(10, None)
                .await
                .map_err(|e| AppError::PubSubError(e.to_string()))?;

            for message in messages {
                handler(message).await;
            }

            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    }
}
