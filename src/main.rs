mod db_processing;
pub mod env_processing;
mod rules;
mod tg_processing;

use crate::db_processing::db_processing::{
    check_user_rec_avail, cr_new_user_rec, get_last_mess_id, get_pgen_rules,
    increase_user_gen_count, init as db_pool_init, set_last_mess_id, update_rules,
};
use crate::env_processing::env_processing::DotEnv;
use log::*;
use passgenlib::Passgen;
use rules::rules::Rules;
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
        //.dependencies(dptree::deps![db_pool])
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
        "➖ include lowercase letters"
    };
    let enab_u_letters = {
        if rules.enab_u_letters {
            "✅ include capital letters"
        } else {
            "➖ include capital letters"
        }
    };
    let enab_num = {
        if rules.enab_num {
            "✅ include numbers"
        } else {
            "➖ include numbers"
        }
    };
    let enab_spec_symbs = {
        if rules.enab_spec_symbs {
            "✅ include special symbols"
        } else {
            "➖ include special symbols"
        }
    };
    let enab_strong_usab = {
        if rules.enab_strong_usab {
            "✅ strong & usability password"
        } else {
            "➖ strong & usability password"
        }
    };
    let custom_charset = {
        if rules.custom_charset.is_empty() {
            "➖ custom charset. Press to set."
        } else {
            "✅ custom charset. Press to set."
        }
    };
    let pwd_len = &format!("{} password length. Press to edit.", rules.pwd_len)[..];

    let inline_btns = [
        [InlineKeyboardButton::callback(enab_letters, "enab_letters")],
        [InlineKeyboardButton::callback(
            enab_u_letters,
            "enab_u_letters",
        )],
        [InlineKeyboardButton::callback(enab_num, "enab_num")],
        [InlineKeyboardButton::callback(
            enab_spec_symbs,
            "enab_spec_symbs",
        )],
        [InlineKeyboardButton::callback(
            enab_strong_usab,
            "enab_strong_usab",
        )],
        [InlineKeyboardButton::callback(
            custom_charset,
            "custom_charset",
        )],
        [InlineKeyboardButton::callback(pwd_len, "pwd_len")],
        [InlineKeyboardButton::callback("▶GENERATE", "generate")],
    ];

    InlineKeyboardMarkup::new(inline_btns)
}

async fn gen_and_send_main_menu(bot: &Bot, chat_id: ChatId, chat_id_i64: i64) {
    let keyboard = main_menu(chat_id_i64).await;

    let mess = bot
        .send_message(chat_id, "<b>⬇ RULE SETTING AND GEN PWD ⬇</b>")
        .parse_mode("HTML".parse().unwrap())
        .reply_markup(keyboard)
        .await;
    let now_str = get_now_str();

    match mess {
        Ok(_ok) => {
            println!("📗 [{now_str}] New menu for user #{chat_id_i64} was sent.");
            let mess_id: i32 = _ok.id.to_string().parse().unwrap();
            set_last_mess_id(chat_id_i64, mess_id, "last_menu_mess_id").await;
        }
        Err(_err) => {
            println!(
                "📕 [{now_str}] Error on sending of new menu for user #{chat_id_i64}: '{_err}'."
            );
        }
    }
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
                let chat_id: ChatId = (&msg.chat.id).to_owned();
                let chat_id_i64: i64 = chat_id.to_string().parse::<i64>().unwrap();
                let now_str = get_now_str();

                if check_user_rec_avail(chat_id_i64).await {
                    gen_and_send_main_menu(&bot, chat_id, chat_id_i64).await;
                } else {
                    match msg.clone().from {
                        Some(from) => {
                            let cr_result = cr_new_user_rec(chat_id_i64, from).await;
                            if cr_result {
                                gen_and_send_main_menu(&bot, chat_id, chat_id_i64).await;
                            } else {
                                bot.send_message(
                                    chat_id,
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
                                chat_id,
                                "⚠️ Sorry, internal service error. Please use the service later.",
                            )
                            .await?;
                        }
                    }
                }
            }
            Err(_) => {
                bot.send_message(msg.chat.id, "🚫 Unknown command!").await?;
            }
        }
    }
    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> Result<(), Box<dyn Error + Send + Sync>> {
    let chat_id: ChatId = q.message.clone().unwrap().chat().id;
    let chat_id_i64: i64 = chat_id.clone().to_string().parse::<i64>().unwrap();
    let action = q.data;
    let mut rules: Rules = get_pgen_rules(chat_id_i64).await.unwrap();

    match action {
        Some(act)
            if act == "enab_letters".to_string()
                || act == "enab_u_letters".to_string()
                || act == "enab_num".to_string()
                || act == "enab_spec_symbs".to_string()
                || act == "enab_strong_usab".to_string() =>
        {
            rules.reconfigure_rules_according_selector(act.clone());
            let set_rules_res = update_rules(chat_id_i64, rules).await;
            if set_rules_res {
                remove_prev_mess(&bot, chat_id, chat_id_i64, "last_gen_mess_id").await;
                remove_prev_mess(&bot, chat_id, chat_id_i64, "last_menu_mess_id").await;
                gen_and_send_main_menu(&bot, chat_id, chat_id_i64).await;
            }
        }
        Some(act) if act == "custom_charset".to_string() => {
            println!("action = {act}")
        }
        Some(act) if act == "pwd_len".to_string() => {
            println!("action = {act}")
        }
        Some(act) if act == "generate".to_string() => {
            remove_prev_mess(&bot, chat_id, chat_id_i64, "last_gen_mess_id").await;

            let mut pgen_from_rules: Passgen = Passgen {
                enab_letters: rules.enab_letters,
                enab_u_letters: rules.enab_u_letters,
                enab_num: rules.enab_num,
                enab_spec_symbs: rules.enab_spec_symbs,
                custom_charset: rules.custom_charset.leak(),
                enab_strong_usab: rules.enab_strong_usab,
            };
            let pwd = pgen_from_rules.generate(rules.pwd_len as u32);

            let mess = bot
                .send_message(chat_id.clone(), format!("<b><code>{pwd}</code></b>"))
                .parse_mode("HTML".parse().unwrap())
                .await?;
            let now_str = get_now_str();
            println!("✅ [{now_str}] New password for user #{chat_id_i64} was sent.");
            let mess_id: i32 = mess.id.to_string().parse().unwrap();

            increase_user_gen_count(chat_id_i64).await;
            set_last_mess_id(chat_id_i64, mess_id, "last_gen_mess_id").await;
        }
        _ => println!("🚫 Unrecognized callback action!"),
    }

    Ok(())
}

async fn remove_prev_mess(bot: &Bot, chat_id: ChatId, chat_id_i64: i64, id_field: &str) {
    let prev_mess_id = get_last_mess_id(chat_id_i64, id_field).await;
    let now_str = get_now_str();
    match prev_mess_id {
        Some(mess_id) => {
            let del_res = bot.delete_message(chat_id, MessageId(mess_id)).await;
            let now_str = get_now_str();
            match del_res {
                Ok(_) => println!(
                    "📗 [{now_str}] Mess #{mess_id} at field {id_field} of user:{} successfully removed.",
                    chat_id.clone().to_string()
                ),
                Err(_) => println!(
                    "📙 [{now_str}] Mess #{mess_id} at field {id_field} of user:{} was not found or already has been removed!",
                    chat_id.clone().to_string()
                ),
            }
        }
        None => println!("📙 [{now_str}] No records to remove \"{id_field}\" found for user #{chat_id_i64}."),
    }
}
