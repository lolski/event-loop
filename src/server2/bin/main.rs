extern crate lib;

use google_cloud_gax::cancel::CancellationToken;
use google_cloud_pubsub::client::Client;
use google_cloud_pubsub::subscriber::ReceivedMessage;

#[tokio::main]
async fn main() {
    let client = Client::default().await.unwrap();
    let topic = lib::topic(&client, "topic-test").await;
    let subscription = lib::subscription(&client, &topic, "subscription-test").await;
    let cancel_token = CancellationToken::new();
    let subscription_handler = |msg: ReceivedMessage, cancel: CancellationToken| async move {
        println!("{:?}", msg.message.data);
    };
    subscription.receive(subscription_handler, cancel_token.clone(), None).await.unwrap();
}