use std::panic;
use std::time::Duration;

use colored::*;
use log::warn;
use tokio::{task, time};
use youtube_chat::{item::ChatItem, live_chat::LiveChatClientBuilder};

const URL: &str = "https://www.youtube.com/watch?v=jfKfPfyJRdk";

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    panic::set_hook(Box::new(|e| {
        if e.to_string().contains("dns error") {
            eprintln!(
                "{}\n\n{}{}",
                "There was a DNS error, did you set the correct URL?".red(),
                "Current URL: ".red(),
                URL.red()
            );
        } else {
            eprintln!("{}", e.to_string().red());
        }
    }));

    let mut client = LiveChatClientBuilder::new()
        .url(URL.to_string())
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
    for message in item.message.iter() {
        match message {
            youtube_chat::item::MessageItem::Text(v) => {
                println!(
                    "{:>10}{:>24} - {v}",
                    item.timestamp
                        .unwrap()
                        .time()
                        .format("%H:%M:%S")
                        .to_string(),
                    item.author.name.clone().unwrap().green()
                )
            }
            youtube_chat::item::MessageItem::Emoji(_) => warn!("Emojis are not supported yet."),
        }
    }
}
