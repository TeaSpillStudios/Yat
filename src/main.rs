use std::panic;
use std::time::Duration;

use tokio::{task, time};
use youtube_chat::{item::ChatItem, live_chat::LiveChatClientBuilder};

#[tokio::main]
async fn main() {
    panic::set_hook(Box::new(|e| {
        if e.to_string().contains("dns error") {
            eprintln!("There was a DNS error, did you set the correct URL?");
        } else {
            eprintln!("{e}");
        }
    }));

    let mut client = LiveChatClientBuilder::new()
        .url("https://www.youtube.com/watch?v=jfKfPfyJRdk".to_string())
        .unwrap()
        .on_chat(|chat_item| handle_message(chat_item))
        .on_error(|error| println!("{error}"))
        .build();

    client.start().await.unwrap();
    let forever = task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(3000));
        loop {
            interval.tick().await;
            client.execute().await;
        }
    });

    forever.await.unwrap();
}

fn handle_message(item: ChatItem) {
    println!("{:?}", item.message);
}
