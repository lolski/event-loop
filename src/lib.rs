use std::time::Duration;
use google_cloud_pubsub::client::Client;
use google_cloud_pubsub::topic::{Topic, TopicConfig};
use google_cloud_pubsub::subscription::{Subscription, SubscriptionConfig};

pub async fn topic(client: &Client, id: &str) -> Topic {
    let topic = client.topic(id);
    if !topic.exists(None, None).await.unwrap() {
        let mut topic_config = TopicConfig::default();
        topic_config.message_retention_duration = Some(Duration::from_secs(600));
        topic.create(
            Some(topic_config),
            None,
            None
        ).await.unwrap();
    }
    topic
}

pub async fn subscription(client: &Client, topic: &Topic, id: &str) -> Subscription {
    let subscription = client.subscription(id);
    if !subscription.exists(None, None).await.unwrap() {
        let mut subscription_config = SubscriptionConfig::default();
        subscription_config.enable_message_ordering = true;
        subscription.create(
            topic.fully_qualified_name(),
            subscription_config,
            None,
            None
        ).await.unwrap();
    }
    subscription
}