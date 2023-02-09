extern crate lib;

use std::thread;
use std::thread::sleep;
use std::time::Duration;
use google_cloud_gax::cancel::CancellationToken;
use google_cloud_pubsub::client::Client;
use google_cloud_pubsub::subscriber::ReceivedMessage;

#[tokio::main]
async fn main() {
    let client = Client::default().await.unwrap();
    let topic = lib::topic(&client, "topic-test").await;
    let subscription = lib::subscription(&client, &topic, "subscription-test").await;
    let cancel_token = CancellationToken::new();
    let cancel_token2 = cancel_token.clone();
    println!("1");
    let subscription_handler = |msg: ReceivedMessage, cancel: CancellationToken| async move {
        println!("- {:?}", msg.message.data);
    };
    println!("2");
    thread::spawn(move || {
        sleep(Duration::from_secs(5));
        cancel_token2.cancel();
        println!("cancel invoked");
    });
    subscription.receive(subscription_handler, cancel_token, None).await.unwrap();
    println!("3");
    subscription.delete(None, None).await.unwrap();
    println!("4");
}