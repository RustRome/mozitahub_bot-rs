use std::env;

use clap::App;
use futures::future::Future;
use futures::stream::Stream;
use log::info;
use mozitahub;
use rutebot::{
    client::Rutebot,
    requests::{GetUpdates, SendMessage},
    responses::{Chat, Message, Update},
};

fn main() {
    let _matches = App::new("mozitahub")
        .author("Rust roma meetup group")
        .version("0.1.0-alpha")
        .get_matches();
    let token =
        env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN environment variable must be present");
    // application config
    let tlgrm_bot = Rutebot::new(token);
    let get_updates = GetUpdates::new_with_timeout(20);
    let bot = tlgrm_bot
        .incoming_updates(get_updates)
        .then(Ok)
        .for_each(move |update| {
            match update {
                Ok(Update {
                    message:
                        Some(Message {
                            message_id,
                            ref chat,
                            text: Some(ref text),
                            ..
                        }),
                    ..
                }) => {
                    println!("message: {} {:?} {}", message_id, chat, text);
                    send_text(&format!("{} from Rust :)", text), &chat, &tlgrm_bot)
                }
                Err(e) => {
                    eprintln!("Update failed: {}", e);
                }
                _ => {}
            };
            Ok(())
        });
    tokio::run(bot);
}

fn send_text(reply_text: &str, chat: &Chat, tlgm_bot: &Rutebot) {
    let respose_message = tlgm_bot.prepare_api_request(SendMessage::new(chat.id, reply_text));
    tokio::spawn(respose_message.send().then(|_| Ok(())));
}
