mod db_processing;
pub mod env_processing;

use crate::db_processing::db_processing::{
    check_user_rec_avail, cr_new_user_rec, get_last_gen_mess_id, get_pgen_rules,
    init as db_pool_init, user_press_inline_btn,
};
use crate::env_processing::env_processing::DotEnv;
use log::*;
use passgenlib::Passgen;
use serde::{Deserialize, Deserializer};
use serde_json::{from_str, json};
use sqlx::postgres::PgColumn;
use sqlx::{Column, Row, ValueRef};
use std::error::Error;
use teloxide::requests::Requester;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, Me};
use teloxide::Bot;
use teloxide::{prelude::*, update_listeners::webhooks, utils::command::BotCommands};
use teloxide_core::types::MessageId;
use teloxide_core::RequestError;

#[derive(BotCommands)]
#[command(rename_rule = "lowercase")]
enum Command {
    /// Select rules and generate password.
    Help,
    /// Main menu
    Start,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct Rules {
    enab_letters: bool,
    enab_u_letters: bool,
    enab_num: bool,
    enab_spec_symbs: bool,
    enab_strong_usab: bool,
    custom_charset: String,
    pwd_len: u64,
}

pub fn get_now_str() -> String {
    chrono::Local::now().format("%d-%b-%y %X%.6f").to_string()
}

#[tokio::main]
async fn main() {
    //Env
    let env_data = DotEnv::parse_dot_env();

    //DB
    db_pool_init(&env_data).await;

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

async fn main_menu(chat_id: i64) -> InlineKeyboardMarkup {
    let rules: Rules = get_pgen_rules(chat_id).await.unwrap();

    let enab_letters = if rules.enab_letters {
        "✅ include lowercase letters"
    } else {
        "☐ include lowercase letters"
    };
    let enab_u_letters = {
        if rules.enab_u_letters {
            "✅ include capital letters"
        } else {
            "☐ include capital letters"
        }
    };
    let enab_num = {
        if rules.enab_num {
            "✅ include numbers"
        } else {
            "☐ include numbers"
        }
    };
    let enab_spec_symbs = {
        if rules.enab_spec_symbs {
            "✅ include special symbols"
        } else {
            "☐ include special symbols"
        }
    };
    let enab_strong_usab = {
        if rules.enab_strong_usab {
            "✅ strong & usability password"
        } else {
            "☐ strong & usability password"
        }
    };
    let custom_charset = {
        if rules.custom_charset.is_empty() {
            "☐ custom charset. Press to set."
        } else {
            "✅ custom charset. Press to set."
        }
    };
    let pwd_len = &format!("Password length is {}. Press to set.", rules.pwd_len)[..];

    let inline_btns = [
        [InlineKeyboardButton::callback(enab_letters, "enab_letters")],
        [InlineKeyboardButton::callback(enab_u_letters, "enab_u_letters")],
        [InlineKeyboardButton::callback(enab_num, "enab_num")],
        [InlineKeyboardButton::callback(enab_spec_symbs, "enab_spec_symbs")],
        [InlineKeyboardButton::callback(enab_strong_usab, "enab_strong_usab")],
        [InlineKeyboardButton::callback(custom_charset, "custom_charset")],
        [InlineKeyboardButton::callback(pwd_len, "pwd_len")],
        [InlineKeyboardButton::callback("▶GENERATE", "generate")],
    ];

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
                let chat_id_i64: i64 = msg.clone().chat.id.to_string().parse::<i64>().unwrap();
                let now_str = get_now_str();

                async fn gen_and_send_main_menu(
                    chat_id: i64,
                    bot: Bot,
                    msg: Message,
                ) -> Result<Message, RequestError> {
                    let keyboard = main_menu(chat_id).await;

                    bot.send_message(msg.clone().chat.id, "⚙<b>Mammothcoding passgen</b>⚙")
                        .parse_mode("HTML".parse().unwrap())
                        .reply_markup(keyboard)
                        .await
                }

                if check_user_rec_avail(chat_id_i64).await {
                    gen_and_send_main_menu(chat_id_i64, bot, msg).await.expect(
                        format!("🚫 [{now_str}] Panic on gen_and_send_main_menu()").as_str(),
                    );
                } else {
                    match msg.clone().from {
                        Some(from) => {
                            let cr_result = cr_new_user_rec(chat_id_i64, from).await;
                            if cr_result {
                                gen_and_send_main_menu(chat_id_i64, bot, msg).await.expect(
                                    format!("🚫 [{now_str}] Panic on gen_and_send_main_menu()")
                                        .as_str(),
                                );
                            } else {
                                bot.send_message(
                                    msg.chat.id,
                                    "⚠️ Sorry, internal service error. Please use the service later.",
                                )
                                    .await?;
                            }
                        }
                        _ => {
                            println!(
                                "📕 [{now_str}] Error on getting msg.from. Chat_id #{chat_id_i64}."
                            );
                            bot.send_message(
                                msg.chat.id,
                                "⚠️ Sorry, internal service error. Please use the service later.",
                            )
                            .await?;
                        }
                    }
                }
            }
            Err(_) => {
                bot.send_message(msg.chat.id, "🚫 Illegal command!").await?;
            }
        }
    }
    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> Result<(), Box<dyn Error + Send + Sync>> {
    let chat_id = q.message.clone().unwrap().chat().id;
    let chat_id_i64: i64 = chat_id.clone().to_string().parse::<i64>().unwrap();

    let prev_gen_mess_id = get_last_gen_mess_id(chat_id_i64).await;
    let now_str = get_now_str();
    match prev_gen_mess_id {
        Some(mess_id) => {
            println!("📗 [{now_str}] Last mess id #{mess_id} for user #{chat_id_i64}.");

            let del_res = bot
                .delete_message(chat_id.clone(), MessageId(mess_id))
                .await;
            let now_str = get_now_str();
            match del_res {
                Ok(_) => println!(
                    "📗 [{now_str}] Mess #{mess_id} of user:{} successfully removed.",
                    chat_id.clone().to_string()
                ),
                Err(_) => println!(
                    "📕 [{now_str}] Mess #{mess_id} of user:{} was not found!",
                    chat_id.clone().to_string()
                ),
            }
        }
        None => println!("📗 [{now_str}] No records found for user #{chat_id_i64}."),
    }

    let pwd = Passgen::default().generate(10);
    let s1 = bot
        .send_message(chat_id.clone(), format!("<b><code>{pwd}</code></b>"))
        .parse_mode("HTML".parse().unwrap())
        .await?;
    let now_str = get_now_str();
    println!("✅ [{now_str}] New password for user #{chat_id_i64} was sent.");
    let s1_i32: i32 = s1.id.to_string().parse().unwrap();

    user_press_inline_btn(chat_id_i64, s1_i32).await;

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
