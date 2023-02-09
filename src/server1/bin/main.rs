use std::time::Duration;
use google_cloud_pubsub::client::Client;
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::topic::TopicConfig;

#[tokio::main]
async fn main() {
    let client = Client::default().await.unwrap();
    // create new topic
    let topic = client.topic("test");
    if topic.exists(None, None).await.unwrap() {
        topic.delete(None, None).await.unwrap();
    }
    let mut topic_config = TopicConfig::default();
    topic_config.message_retention_duration = Some(Duration::from_secs(600));
    topic.create(
        Some(topic_config),
        None,
        None
    ).await.unwrap();

    let publisher = topic.new_publisher(None);
    let mut message = PubsubMessage::default();
    message.data = "test-message".into();
    let publish = publisher.publish(message).await;
    publish.get(None).await.unwrap();
    // how to delete existing topic?
}
