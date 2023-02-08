use google_cloud_pubsub::client::{Client, ClientConfig};

#[tokio::main]
async fn main() {
    let mut client = Client::default().await.unwrap();
    let topic = client.topic("test");
    if !topic.exists(None, None) {
        topic.create(None, None, None);
    }
    topic.subscriptions(None, None, None).await;

}