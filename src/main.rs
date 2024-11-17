use std::path;
use std::path::PathBuf;
use log::*;
use teloxide::dispatching::{Dispatcher, HandlerExt, UpdateFilterExt};
use teloxide::dptree::{case, endpoint};
use teloxide::error_handlers::LoggingErrorHandler;
use teloxide::{prelude::*, utils::command::BotCommands, update_listeners::webhooks};
use teloxide::requests::Requester;
use teloxide::{dptree, respond, Bot};
use teloxide::payloads::SendPhoto;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile, ReplyMarkup};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::new("7745281341:AAFWcFzoCL0KK2HjyblVN7n1zQL4AUvHsD0");
    //dbg!(bot.get_me().await).expect("TODO: panic message");


    let addr = ([0, 0, 0, 0], 8000).into();
    let url = "https://tg.passgen.mamont.xyz/telegram/webhooking".parse().unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");


    teloxide::repl_with_listener(
        bot,
        |bot: Bot, msg: Message| async move {
            //let logo = "<a href=\"https://passgen.mamont.xyz/McDev_640x360.png\">logo</a>";
            let inline_btns = ["⚙Generate password"]
                .map(|btn| InlineKeyboardButton::callback(btn, btn));
            println!("{}", msg.text().clone().unwrap().to_string());

            bot.send_message(msg.chat.id, "<b>Menu:</b>")
                .parse_mode("HTML".parse().unwrap())
                .reply_markup(InlineKeyboardMarkup::new([inline_btns]))
                .await?;
            bot.send_message(msg.chat.id, "pwd-bla-bla").await?;


            Ok(())
        },
        listener,
    )
        .await;

}
