extern crate lib;

use google_cloud_pubsub::client::Client;
use google_cloud_googleapis::pubsub::v1::PubsubMessage;

#[tokio::main]
async fn main() {
    let client = Client::default().await.unwrap();
    let topic = lib::topic(&client, "topic-test").await;
    let publisher = topic.new_publisher(None);
    let mut message = PubsubMessage::default();
    message.data = "message-test".into();
    let publish = publisher.publish(message).await;
    publish.get(None).await.unwrap();
    // how to delete existing topic?
}
