use env_file_reader::read_file;
use log::*;
use std::{env};
use passgenlib::Passgen;
use teloxide::requests::Requester;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile, ReplyMarkup};
use teloxide::{Bot};
use teloxide::{prelude::*, update_listeners::webhooks, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    // Log.
    /*pretty_env_logger::init();
    log::info!("Starting command bot...");*/

    /// Get variables from ".env" file stored near binary file.
    let binding = env::current_exe().unwrap();
    let env_path = binding.with_file_name(".env");
    let env_variables = read_file(env_path).expect("Could not load .env file");
    let tg_token: &String = &env_variables["TELEGRAM_BOT_TOKEN"];
    let tg_webhook_url = (&env_variables["TELEGRAM_WEBHOOK_URL"] as &str)
        .parse()
        .unwrap();
    let socket_addr = (&env_variables["TELEGRAM_BOT_SOCKET_ADDR"] as &str)
        .parse()
        .unwrap();

    /// Setup listener.
    //let bot = Bot::from_env();
    let bot: Bot = Bot::new(tg_token);
    let listener = webhooks::axum(
        bot.clone(),
        webhooks::Options::new(socket_addr, tg_webhook_url),
    )
    .await
    .expect("Couldn't setup webhook");

    teloxide::repl_with_listener(
        bot,
        |bot: Bot, msg: Message| async move {
            let inline_btns = ["⚙SETTINGS", "GENERATE↩"].map(|btn| InlineKeyboardButton::callback(btn, btn));
            println!("{}", msg.text().clone().unwrap().to_string());

            bot.send_message(msg.chat.id, "<b>Menu:</b>")
                .parse_mode("HTML".parse().unwrap())
                .reply_markup(InlineKeyboardMarkup::new([inline_btns]))
                .await?;
            bot.send_message(msg.chat.id, Passgen::default_strong_and_usab().generate(10)).await?;

            Ok(())
        },
        listener,
    )
    .await;
}
