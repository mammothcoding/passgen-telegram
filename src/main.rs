use env_file_reader::read_file;
use log::*;
use passgenlib::Passgen;
use std::env;
use std::error::Error;
use teloxide::requests::Requester;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResultArticle, InputFile, InputMessageContent, InputMessageContentText, Me, ReplyMarkup};
use teloxide::Bot;
use teloxide::{prelude::*, update_listeners::webhooks, utils::command::BotCommands};

#[derive(BotCommands)]
#[command(rename_rule = "lowercase")]
enum Command {
    /// Choose your destiny
    Help,
    /// Start
    Start,
}

#[tokio::main]
async fn main() {// -> Result<(), Box<dyn Error>> {
    // Log.
    /*pretty_env_logger::init();
    log::info!("Starting command bot...");*/

    // Get variables from ".env" file stored near binary file.
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

    // Setup listener.
    let bot: Bot = Bot::new(tg_token);
    let listener = webhooks::axum(
        bot.clone(),
        webhooks::Options::new(socket_addr, tg_webhook_url),
    )
    .await
    .expect("Couldn't setup webhook");

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await;
    //Ok(())

    //teloxide::repl_with_listener(bot, handler, listener).await;

    /*teloxide::repl_with_listener(
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
    .await;*/
}

fn make_keyboard() -> InlineKeyboardMarkup {
    let settings_btn = InlineKeyboardButton::callback("⚙SETTINGS", "settings");
    let gen_btn = InlineKeyboardButton::callback("▶GENERATE", "generate");
    let inline_btns = [[settings_btn], [gen_btn]];

    InlineKeyboardMarkup::new(inline_btns)
}

async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Help) => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
            }
            Ok(Command::Start) => {
                let keyboard = make_keyboard();
                bot.send_message(msg.chat.id, "<b>Menu:</b>")
                    .parse_mode("HTML".parse().unwrap())
                    .reply_markup(keyboard)
                    .await?;
            }
            Err(_) => {
                bot.send_message(msg.chat.id, "🚫Illegal command!").await?;
            }
        }
    }
    Ok(())
}

async fn inline_query_handler(
    bot: Bot,
    q: InlineQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let choose_debian_version = InlineQueryResultArticle::new(
        "0",
        "Chose debian version",
        InputMessageContent::Text(InputMessageContentText::new("Debian versions:")),
    )
        .reply_markup(make_keyboard());

    bot.answer_inline_query(q.id, vec![choose_debian_version.into()]).await?;

    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(q.message.unwrap().chat().id, Passgen::default_strong_and_usab().generate(10)).await?;

    if let Some(ref version) = q.data {
        let text = format!("You chose: {version}");
        bot.answer_callback_query(&q.id).await?;
        if let Some(id) = q.inline_message_id {
            bot.edit_message_text_inline(id, text).await?;
        }
    }

    /*if let Some(ref version) = q.data {
        let text = format!("You chose: {version}");

        // Tell telegram that we've seen this query, to remove 🕑 icons from the
        // clients. You could also use `answer_callback_query`'s optional
        // parameters to tweak what happens on the client side.
        bot.answer_callback_query(&q.id).await?;

        // Edit text of the message to which the buttons were attached
        if let Some(message) = q.regular_message() {
            bot.edit_message_text() .edit_text(message, text).await?;
        } else if let Some(id) = q.inline_message_id {
            bot.edit_message_text_inline(id, text).await?;
        }
    }*/

    Ok(())
}
