mod db_processing;
pub mod env_processing;
mod log;
mod rules;
mod tg_processing;
mod user;

use crate::db_processing::db_processing::{
    check_user_rec_avail, cr_new_user_rec, get_last_mess_id, get_pgen_rules,
    get_user_dialog_context, increase_user_gen_count, init as db_pool_init, set_last_mess_id,
    set_user_dialog_context, update_rules,
};
use crate::env_processing::env_processing::DotEnv;
use passgenlib::Passgen;
use rules::rules::Rules;
use std::error::Error;
use ::log::{debug, error, info, trace, warn};
use teloxide::requests::Requester;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, Me};
use teloxide::Bot;
use teloxide::{prelude::*, update_listeners::webhooks, utils::command::BotCommands};
use teloxide_core::types::{KeyboardButton, KeyboardMarkup, MessageId};
use crate::log::log::init as log_init;

#[derive(BotCommands)]
#[command(rename_rule = "lowercase")]
enum Command {
    ///
    /// 🔏 Telegram bot-service for generating cryptographically secure passwords/tokens and other sets and sequences.
    ///
    /// 🔹 You can create a regular password,
    /// choosing in the rules the presence of small and capital letters, numbers, special characters.
    ///
    /// 🔹 You can create a strong and usability password:
    /// Including all characters, but
    /// the first position in the password is a capital or small letter,
    /// the last position is the symbol. Excluded ambiguous characters "0oOiIlL1".
    /// 🔸 If this rule is enabled, the other consistency rules of the generating are not taken,
    /// except for a rule "custom charset".
    ///
    /// 🔹 You can create a set from your custom charset:
    /// 🔸 This set of characters will exclude all other rules except for a rule "strong & usability password".
    /// ⚙️ If "strong & usability password" on too then you can generate combined strong and usability result with custom charset.
    ///
    /// 🔹 You can specify the required password length of not less than 4 and not more than 3900.
    ///
    /// 🦀 Made with Rust.
    /// 🔗 Homepage of this project: "https://github.com/mammothcoding/passgen-telegram".
    ///
    Help,
    /// 📱 Main menu
    Start,
}

pub fn get_now_str() -> String {
    chrono::Local::now().format("%d-%b-%y %X%.6f %Z").to_string()
}

#[tokio::main]
async fn main() {
    // Env
    let env_data = DotEnv::parse_dot_env();

    // Log
    log_init(&env_data);
    warn!("HOP HOP HOP HOP HOP HOP HOP HOP HOP HOP HOP HOP HOP HOP HOP HOP HOP HOP HOP HOP HOP HOP HOP HOP HOP ");
    error!("error");
    warn!("warn");
    info!("info");
    debug!("debug");
    trace!("trace");

    // DB
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
    let pwd_len = &format!("password length 🟰 {} . Press to edit.", rules.pwd_len)[..];

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
        [InlineKeyboardButton::callback("🎲GENERATE", "generate")],
    ];

    InlineKeyboardMarkup::new(inline_btns)
}

async fn gen_and_send_main_menu(bot: &Bot, chat_id: ChatId, chat_id_i64: i64) {
    let keyboard = main_menu(chat_id_i64).await;

    let mess = bot
        .send_message(chat_id, "<b>⚙ <u>MAMMOTHCODING PASSGEN</u></b>")
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
        let chat_id: ChatId = (&msg.chat.id).to_owned();
        let chat_id_i64: i64 = chat_id.to_string().parse::<i64>().unwrap();
        let bot_id_i64: i64 = (&me.id).to_owned().to_string().parse::<i64>().unwrap();

        match BotCommands::parse(text, me.username()) {
            Ok(Command::Help) => {
                let now_str = get_now_str();
                println!("📗 [{now_str}] User #{chat_id_i64} enter command /Help.");

                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?;
                set_user_dialog_context(chat_id_i64, "NULL").await;
            }
            Ok(Command::Start) => {
                let now_str = get_now_str();
                println!("📗 [{now_str}] User #{chat_id_i64} enter command /Start.");

                if check_user_rec_avail(chat_id_i64).await {
                    remove_prev_mess(&bot, chat_id, chat_id_i64, "last_gen_mess_id").await;
                    remove_prev_mess(&bot, chat_id, chat_id_i64, "last_menu_mess_id").await;
                    gen_and_send_main_menu(&bot, chat_id, chat_id_i64).await;
                    set_user_dialog_context(chat_id_i64, "NULL").await;
                } else {
                    match msg.from {
                        Some(from) => {
                            let cr_result = cr_new_user_rec(chat_id_i64, bot_id_i64, from).await;
                            if cr_result {
                                gen_and_send_main_menu(&bot, chat_id, chat_id_i64).await;
                                set_user_dialog_context(chat_id_i64, "NULL").await;
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
            // Unknown command or text for necessary context or necessary text action
            Err(_) => {
                let context = get_user_dialog_context(chat_id_i64).await;
                match context {
                    Some(context) if context == "custom_charset".to_string() => {
                        match text.parse::<String>() {
                            Ok(c_chset) => {
                                if c_chset.len() < 1000 {
                                    let mut rules: Rules =
                                        get_pgen_rules(chat_id_i64).await.unwrap();
                                    rules.custom_charset = c_chset;
                                    rules.reconfigure_rules_according_selector(context);
                                    let set_rules_res = update_rules(chat_id_i64, rules).await;

                                    let now_str = get_now_str();
                                    if set_rules_res {
                                        set_user_dialog_context(chat_id_i64, "NULL").await;
                                        remove_prev_mess(
                                            &bot,
                                            chat_id,
                                            chat_id_i64,
                                            "last_gen_mess_id",
                                        )
                                        .await;
                                        remove_prev_mess(
                                            &bot,
                                            chat_id,
                                            chat_id_i64,
                                            "last_menu_mess_id",
                                        )
                                        .await;
                                        gen_and_send_main_menu(&bot, chat_id, chat_id_i64).await;
                                        println!(
                                            "📗 [{now_str}] Custom_charset for user #{chat_id_i64} successfully set."
                                        )
                                    } else {
                                        println!(
                                            "📕 [{now_str}] Error by set custom_charset for user #{chat_id_i64}!"
                                        );
                                        bot.send_message(
                                            chat_id,
                                            "⚠️ Sorry, internal service error on set custom charset. Please use the service later.",
                                        )
                                            .await?;
                                    }
                                } else {
                                    let now_str = get_now_str();
                                    println!(
                                        "📕 [{now_str}] A very large charset size has been passed #{chat_id_i64}!"
                                    );
                                    bot.send_message(
                                        chat_id,
                                        "<i>⚠️ A very large custom charset size has been passed. Please enter your character set below again 🔡🔢🔣</i>",
                                    )
                                        .parse_mode("HTML".parse().unwrap())
                                        .await?;
                                }
                            }
                            Err(_err) => {
                                let now_str = get_now_str();
                                println!(
                                    "📙 [{now_str}] Error on parsing mess for set custom_charset from user #{chat_id_i64}. Received text: {text}"
                                );
                                bot
                                    .send_message(chat_id, "⚠️ Sorry, internal service error on set custom charset. Please use the service later.")
                                    .await?;
                            }
                        }
                    }
                    Some(context) if context == "pwd_len".to_string() => {
                        match text.parse::<u64>() {
                            Ok(mut pwd_len) => {
                                if pwd_len > 3900 {
                                    pwd_len = 3900;
                                }
                                if pwd_len < 4 {
                                    pwd_len = 4;
                                }
                                let mut rules: Rules = get_pgen_rules(chat_id_i64).await.unwrap();
                                rules.pwd_len = pwd_len;
                                let set_rules_res = update_rules(chat_id_i64, rules).await;

                                let now_str = get_now_str();
                                if set_rules_res {
                                    set_user_dialog_context(chat_id_i64, "NULL").await;
                                    remove_prev_mess(
                                        &bot,
                                        chat_id,
                                        chat_id_i64,
                                        "last_gen_mess_id",
                                    )
                                    .await;
                                    remove_prev_mess(
                                        &bot,
                                        chat_id,
                                        chat_id_i64,
                                        "last_menu_mess_id",
                                    )
                                    .await;
                                    gen_and_send_main_menu(&bot, chat_id, chat_id_i64).await;
                                    println!(
                                        "📗 [{now_str}] Pwd_len for user #{chat_id_i64} successfully set."
                                    )
                                } else {
                                    println!(
                                        "📕 [{now_str}] Error by set pwd_len for user #{chat_id_i64}!"
                                    );
                                    bot.send_message(
                                        chat_id,
                                        "⚠️ Sorry, internal service error on set password length. Please use the service later.",
                                    )
                                        .await?;
                                }
                            }
                            Err(_err) => {
                                let now_str = get_now_str();
                                println!(
                                    "📙 [{now_str}] Unknown type in mess from user #{chat_id_i64}. Received text: {text}"
                                );
                                bot
                                    .send_message(chat_id, "🚫 <i>Wrong number! Please enter your password length again🔢</i>")
                                    .parse_mode("HTML".parse().unwrap())
                                    .await?;
                            }
                        }
                    }
                    None => {}
                    _ => {
                        if text == "🧹 pwd" {
                            remove_prev_mess(&bot, chat_id, chat_id_i64, "last_gen_mess_id").await;
                            bot.delete_message(chat_id, msg.id).await?;
                        } else {
                            let now_str = get_now_str();
                            bot.send_message(chat_id, "🚫 Unknown command!").await?;
                            println!(
                                "📙 [{now_str}] Unknown command from user #{}. Command text: {text}",
                                msg.chat.id
                            );
                        }
                    }
                }
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

    set_user_dialog_context(chat_id_i64, "NULL").await;

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
            if rules.custom_charset == "" {
                set_user_dialog_context(chat_id_i64, "custom_charset").await;
                bot.send_message(
                    chat_id.clone(),
                    "<i>Please enter your character set below to generate a password
🔡🔢🔣</i>",
                )
                .parse_mode("HTML".parse().unwrap())
                .await?;
            } else {
                rules.custom_charset = "".to_string();
                rules.reconfigure_rules_according_selector(act.clone());
                let set_rules_res = update_rules(chat_id_i64, rules).await;
                if set_rules_res {
                    remove_prev_mess(&bot, chat_id, chat_id_i64, "last_gen_mess_id").await;
                    remove_prev_mess(&bot, chat_id, chat_id_i64, "last_menu_mess_id").await;
                    gen_and_send_main_menu(&bot, chat_id, chat_id_i64).await;
                }
            }
        }
        Some(act) if act == "pwd_len".to_string() => {
            set_user_dialog_context(chat_id_i64, "pwd_len").await;
            bot.send_message(
                chat_id.clone(),
                "<i>Please enter your password length below🔢</i>",
            )
            .parse_mode("HTML".parse().unwrap())
            .await?;
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
                .send_message(
                    chat_id.clone(),
                    format!("<i>Password is (click to copy): </i><b><code>{pwd}</code></b>"),
                )
                .parse_mode("HTML".parse().unwrap())
                .reply_markup(
                    KeyboardMarkup::new([[KeyboardButton::new("🧹 pwd")]])
                        .one_time_keyboard()
                        .resize_keyboard(),
                )
                .await?;
            let now_str = get_now_str();
            println!("🎲 [{now_str}] New password for user #{chat_id_i64} was sent.");
            let mess_id: i32 = mess.id.to_string().parse().unwrap();

            increase_user_gen_count(chat_id_i64).await;
            set_last_mess_id(chat_id_i64, mess_id, "last_gen_mess_id").await;
        }
        _ => {
            let now_str = get_now_str();
            println!("📙 [{now_str}] Unrecognized callback action!");
        }
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
        None => println!(
            "📙 [{now_str}] No records to remove \"{id_field}\" found for user #{chat_id_i64}."
        ),
    }
}
