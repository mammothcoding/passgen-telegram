mod db_processing;
pub mod env_processing;

use crate::db_processing::db_processing::{init as db_pool_init, user_press_inline_btn};
use crate::env_processing::env_processing::DotEnv;
use log::*;
use passgenlib::Passgen;
use std::error::Error;
use std::process;
use sqlx::{Pool, Postgres};
use teloxide::requests::Requester;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, Me};
use teloxide::Bot;
use teloxide::{prelude::*, update_listeners::webhooks, utils::command::BotCommands};
use teloxide_core::types::MessageId;
use tokio::sync::OnceCell;

#[derive(BotCommands)]
#[command(rename_rule = "lowercase")]
enum Command {
    /// Choose your destiny
    Help,
    /// Main menu
    Start,
}

//static ENV_DATA: DotEnv = DotEnv::parse_dot_env();

static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    //Env
    let env_data = DotEnv::parse_dot_env();

    //DB
    //static DB_POOL: Pool<Postgres> = db_pool_init(&env_data).await;
    //DB_POOL: Pool<Postgres> = db_pool_init(&env_data).await;
    DB_POOL.get_or_init(|| async {
        db_pool_init(&env_data).await
    })
        .await;

    // Setup listener.
    let bot: Bot = Bot::new(&env_data.tg_bot_token);
    let listener = webhooks::axum(
        bot.clone(),
        webhooks::Options::new(
            (&env_data.tg_bot_socket_addr).parse().unwrap(),
            (&env_data.tg_web_hook_url).parse().unwrap(),
        ),
    )
    .await
    .expect("🚫 Couldn't setup webhook");

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("🚫 An error from the update listener"),
        )
        .await;
}

fn main_menu() -> InlineKeyboardMarkup {
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
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?;
            }
            Ok(Command::Start) => {
                let keyboard = main_menu();
                bot.send_message(msg.chat.id, "<b>Main menu:</b>")
                    .parse_mode("HTML".parse().unwrap())
                    .reply_markup(keyboard)
                    .await?;
            }
            Err(_) => {
                bot.send_message(msg.chat.id, "🚫 Illegal command!").await?;
            }
        }
    }
    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> Result<(), Box<dyn Error + Send + Sync>> {
    let prev_gen_mess: i32 = 265;
    let chat_id = q.message.clone().unwrap().chat().id;
    let chat_id_u64: u64 = chat_id.clone().to_string().parse::<u64>().unwrap();

    /*let del_res = bot
        .delete_message(chat_id.clone(), MessageId(prev_gen_mess))
        .await;
    match del_res {
        Ok(_) => println!(
            "📗 Mess #{} of user:{} successfully removed",
            prev_gen_mess,
            chat_id.clone().to_string()
        ),
        Err(_) => println!(
            "📕 Mess #{} of user:{} was not found!",
            prev_gen_mess,
            chat_id.clone().to_string()
        ),
    }*/

    let s1 = bot
        .send_message(
            chat_id.clone(),
            Passgen::default_strong_and_usab().generate(10),
        )
        .await?;
    let s1_i32: i32 = s1.id.to_string().parse().unwrap();

    user_press_inline_btn(DB_POOL.get().expect("DB_POOL.get().expect"), chat_id_u64, s1_i32).await;

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
