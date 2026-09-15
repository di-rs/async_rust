use futures::StreamExt;
use mini_redis::{Result, client};
use std::future;

async fn publish() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "two".into()).await?;
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "four".into()).await?;
    client.publish("numbers", "five".into()).await?;
    client.publish("numbers", "6".into()).await?;

    Ok(())
}

async fn subscribe() -> Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;
    let subscriber = client.subscribe(vec!["numbers".to_owned()]).await?;
    let messages = subscriber
        .into_stream()
        .filter_map(async |msg| msg.ok())
        .filter(|msg| future::ready(msg.content.len() == 1))
        .map(|msg| msg.content)
        .take(3);

    tokio::pin!(messages);

    while let Some(msg) = messages.next().await {
        println!("got = {msg:?}");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tokio::spawn(async { publish().await });

    subscribe().await?;

    println!("DONE!");

    Ok(())
}
