use log::*;
use teloxide::dispatching::{Dispatcher, HandlerExt, UpdateFilterExt};
use teloxide::dptree::{case, endpoint};
use teloxide::error_handlers::LoggingErrorHandler;
use teloxide::{prelude::*, utils::command::BotCommands, update_listeners::webhooks};
use teloxide::requests::Requester;
use teloxide::{dptree, respond, Bot};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::new("7745281341:AAFWcFzoCL0KK2HjyblVN7n1zQL4AUvHsD0");
    //dbg!(bot.get_me().await).expect("TODO: panic message");

    let addr = ([192, 168, 0, 10], 8000).into();
    let url = "https://passgen.mamont.xyz/telegram/webhooking".parse().unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");

    teloxide::repl_with_listener(
        bot,
        |bot: Bot, msg: Message| async move {
            bot.send_message(msg.chat.id, "hop").await?;
            Ok(())
        },
        listener,
    )
        .await;

}
