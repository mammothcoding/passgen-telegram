pub mod tg_processing {
    use crate::db_processing::db_processing::{
        check_user_rec_avail, cr_new_user_rec, get_last_mess_id, get_pgen_rules,
        get_user_dialog_context, increase_user_gen_count, set_last_mess_id, set_user_app_lang,
        set_user_dialog_context, update_rules,
    };
    use crate::get_now_str;
    use crate::lang_processing::lang_processing::get_lang_map;
    use crate::rules::rules::Rules;
    use log::{debug, error, info, warn};
    use passgenlib::Passgen;
    use std::collections::HashMap;
    use std::error::Error;
    use teloxide::utils::command::BotCommands;
    use teloxide_core::payloads::SendMessageSetters;
    use teloxide_core::prelude::{ChatId, Requester};
    use teloxide_core::types::{
        CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup,
        Me, Message, MessageId,
    };
    use teloxide_core::Bot;

    #[derive(BotCommands)]
    #[command(rename_rule = "lowercase")]
    enum Command {
        Help,
        Start,
        Ru,
        En,
        Es,
        Pt,
        Fr,
        De,
    }

    pub async fn send_test_tg_mess(bot: &Bot) {
        let test_mess = bot.get_me().await;
        let now_str = get_now_str();
        match test_mess {
            Ok(_) => {
                println!("[{now_str}] ✅ - TG bot startup OK.");
                info!("✅ TG bot start OK.");
            }
            Err(err) => {
                println!(
                    "[{now_str}] 🚫 - Error during startup testing of TG bot: {}",
                    err
                );
                error!("🚫 Error during startup testing of TG bot: {}", err);
            }
        }
    }

    async fn main_menu(chat_id: i64, user_lang_map: HashMap<&str, &str>) -> InlineKeyboardMarkup {
        let rules: Rules = get_pgen_rules(chat_id).await.unwrap();

        let enab_letters = if rules.enab_letters {
            &format!("✅ {}", user_lang_map["menu_lcase"])[..]
        } else {
            &format!("➖ {}", user_lang_map["menu_lcase"])[..]
        };
        let enab_u_letters = {
            if rules.enab_u_letters {
                &format!("✅ {}", user_lang_map["menu_cap"])[..]
            } else {
                &format!("➖ {}", user_lang_map["menu_cap"])[..]
            }
        };
        let enab_num = {
            if rules.enab_num {
                &format!("✅ {}", user_lang_map["menu_num"])[..]
            } else {
                &format!("➖ {}", user_lang_map["menu_num"])[..]
            }
        };
        let enab_spec_symbs = {
            if rules.enab_spec_symbs {
                &format!("✅ {}", user_lang_map["menu_ss"])[..]
            } else {
                &format!("➖ {}", user_lang_map["menu_ss"])[..]
            }
        };
        let enab_strong_usab = {
            if rules.enab_strong_usab {
                &format!("✅ {}", user_lang_map["menu_conven"])[..]
            } else {
                &format!("➖ {}", user_lang_map["menu_conven"])[..]
            }
        };
        let custom_charset = {
            if rules.custom_charset.is_empty() {
                &format!("➖ {}", user_lang_map["menu_cch"])[..]
            } else {
                &format!("✅ {}", user_lang_map["menu_cch"])[..]
            }
        };
        let pwd_len = &format!(
            "{} 🟰 {} {}",
            user_lang_map["menu_pass_len1"], rules.pwd_len, user_lang_map["menu_pass_len2"]
        )[..];

        let pwd_quantity = &format!(
            "{} 🟰 {} {}",
            user_lang_map["menu_pass_qua"], rules.pwd_quantity, user_lang_map["menu_pass_len2"]
        )[..];

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
            [InlineKeyboardButton::callback(pwd_quantity, "pwd_quantity")],
            [InlineKeyboardButton::callback(
                user_lang_map["menu_btn_gen"],
                "generate",
            )],
        ];

        InlineKeyboardMarkup::new(inline_btns)
    }

    async fn gen_and_send_main_menu(
        bot: &Bot,
        chat_id: ChatId,
        chat_id_i64: i64,
        user_lang_map: HashMap<&str, &str>,
    ) {
        let keyboard = main_menu(chat_id_i64, user_lang_map).await;

        let mess = bot
            .send_message(
                chat_id,
                "⚙ <b>McPassgen</b>               /help❔\n🌐 /en  /es  /pt  /fr  /de  /ru",
            )
            .parse_mode("HTML".parse().unwrap())
            .reply_markup(keyboard)
            .await;

        match mess {
            Ok(_ok) => {
                debug!("📗 New menu for user #{chat_id_i64} was sent.");
                let mess_id: i32 = _ok.id.to_string().parse().unwrap();
                set_last_mess_id(chat_id_i64, mess_id, "last_menu_mess_id").await;
            }
            Err(_err) => {
                warn!("📕 Error on sending of new menu for user #{chat_id_i64}: '{_err}'.");
            }
        }
    }

    pub async fn message_handler(
        bot: Bot,
        msg: Message,
        me: Me,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(text) = msg.text() {
            let chat_id: ChatId = (&msg.chat.id).to_owned();
            let chat_id_i64: i64 = chat_id.to_string().parse::<i64>().unwrap();
            let bot_id_i64: i64 = (&me.id).to_owned().to_string().parse::<i64>().unwrap();
            let user_lang_map: HashMap<&str, &str> = get_lang_map(chat_id_i64).await;

            match BotCommands::parse(text, me.username()) {
                Ok(Command::Help) => {
                    debug!("📗 User #{chat_id_i64} enter command /help.");
                    bot.send_message(chat_id, user_lang_map["help"])
                        .parse_mode("HTML".parse().unwrap())
                        .await?;
                    set_user_dialog_context(chat_id_i64, "NULL").await;
                }
                Ok(Command::Start) => {
                    debug!("📗 User #{chat_id_i64} enter command /start.");
                    if check_user_rec_avail(chat_id_i64).await {
                        remove_prev_mess(&bot, chat_id, chat_id_i64, "last_gen_mess_id").await;
                        remove_prev_mess(&bot, chat_id, chat_id_i64, "last_menu_mess_id").await;
                        gen_and_send_main_menu(&bot, chat_id, chat_id_i64, user_lang_map).await;
                        set_user_dialog_context(chat_id_i64, "NULL").await;
                    } else {
                        match msg.from {
                            Some(from) => {
                                let cr_result =
                                    cr_new_user_rec(chat_id_i64, bot_id_i64, from).await;
                                if cr_result {
                                    let user_lang_map: HashMap<&str, &str> =
                                        get_lang_map(chat_id_i64).await;
                                    gen_and_send_main_menu(
                                        &bot,
                                        chat_id,
                                        chat_id_i64,
                                        user_lang_map,
                                    )
                                    .await;
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
                                warn!("📕 Error on getting \"msg.from\" data. Chat_id #{chat_id_i64}.");
                                bot.send_message(
                                    chat_id,
                                    "⚠️ Sorry, internal service error. Please use the service later.",
                                )
                                    .await?;
                            }
                        }
                    }
                }
                Ok(Command::En) => change_iface_lang(bot, chat_id, chat_id_i64, "en").await,
                Ok(Command::Es) => change_iface_lang(bot, chat_id, chat_id_i64, "es").await,
                Ok(Command::Pt) => change_iface_lang(bot, chat_id, chat_id_i64, "pt").await,
                Ok(Command::Fr) => change_iface_lang(bot, chat_id, chat_id_i64, "fr").await,
                Ok(Command::De) => change_iface_lang(bot, chat_id, chat_id_i64, "de").await,
                Ok(Command::Ru) => change_iface_lang(bot, chat_id, chat_id_i64, "ru").await,
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
                                            gen_and_send_main_menu(
                                                &bot,
                                                chat_id,
                                                chat_id_i64,
                                                user_lang_map,
                                            )
                                            .await;
                                            debug!(
                                            "📗 Custom_charset for user #{chat_id_i64} successfully set."
                                        )
                                        } else {
                                            warn!(
                                            "📕 Error by set custom_charset for user #{chat_id_i64}!"
                                        );
                                            bot.send_message(
                                                chat_id,
                                                "⚠️ Sorry, internal service error on set custom charset. Please use the service later.",
                                            )
                                                .await?;
                                        }
                                    } else {
                                        info!(
                                        "📙 A very large charset size has been passed by user #{chat_id_i64}!"
                                    );
                                        bot.send_message(
                                            chat_id,
                                            user_lang_map["dialog_large_cch"],
                                        )
                                        .parse_mode("HTML".parse().unwrap())
                                        .await?;
                                    }
                                }
                                Err(_err) => {
                                    warn!(
                                    "📙 Error on parsing mess for set custom_charset from user #{chat_id_i64}. Received text: {text}"
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
                                    let mut rules: Rules =
                                        get_pgen_rules(chat_id_i64).await.unwrap();
                                    rules.pwd_len = pwd_len;
                                    let set_rules_res = update_rules(chat_id_i64, rules).await;

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
                                        gen_and_send_main_menu(
                                            &bot,
                                            chat_id,
                                            chat_id_i64,
                                            user_lang_map,
                                        )
                                        .await;
                                        debug!(
                                            "📗 Pwd_len for user #{chat_id_i64} successfully set."
                                        )
                                    } else {
                                        error!("📕 Error by set pwd_len for user #{chat_id_i64}!");
                                        bot.send_message(
                                            chat_id,
                                            "⚠️ Sorry, internal service error on set password length. Please use the service later.",
                                        )
                                            .await?;
                                    }
                                }
                                Err(_err) => {
                                    debug!(
                                    "📙 Unknown type in mess from user #{chat_id_i64}. Received text: {text}"
                                );
                                    bot.send_message(chat_id, user_lang_map["dialog_wrng_plen"])
                                        .parse_mode("HTML".parse().unwrap())
                                        .await?;
                                }
                            }
                        }
                        Some(context) if context == "pwd_quantity".to_string() => {
                            match text.parse::<u64>() {
                                Ok(mut pwd_quantity) => {
                                    if pwd_quantity > 50 {
                                        pwd_quantity = 50;
                                    }
                                    if pwd_quantity < 1 {
                                        pwd_quantity = 1;
                                    }
                                    let mut rules: Rules =
                                        get_pgen_rules(chat_id_i64).await.unwrap();
                                    rules.pwd_quantity = pwd_quantity;
                                    let set_rules_res = update_rules(chat_id_i64, rules).await;

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
                                        gen_and_send_main_menu(
                                            &bot,
                                            chat_id,
                                            chat_id_i64,
                                            user_lang_map,
                                        )
                                        .await;
                                        debug!(
                                            "📗 Pwd_quantity for user #{chat_id_i64} successfully set."
                                        )
                                    } else {
                                        error!(
                                            "📕 Error by set pwd_quantity for user #{chat_id_i64}!"
                                        );
                                        bot.send_message(
                                            chat_id,
                                            "⚠️ Sorry, internal service error on set password quantity. Please use the service later.",
                                        )
                                            .await?;
                                    }
                                }
                                Err(_err) => {
                                    debug!(
                                    "📙 Unknown type in mess from user #{chat_id_i64}. Received text: {text}"
                                );
                                    bot.send_message(chat_id, user_lang_map["dialog_wrng_plen"])
                                        .parse_mode("HTML".parse().unwrap())
                                        .await?;
                                }
                            }
                        }
                        None => {}
                        _ => {
                            if text == "🧹 pwd" {
                                remove_prev_mess(&bot, chat_id, chat_id_i64, "last_gen_mess_id")
                                    .await;
                                bot.delete_message(chat_id, msg.id).await?;
                                debug!("📗 User #{chat_id_i64} send \"🧹 pwd\". Last mess with pwd was deleted.");
                            } else {
                                bot.send_message(chat_id, user_lang_map["dialog_unk_cmd"])
                                    .parse_mode("HTML".parse().unwrap())
                                    .await?;
                                info!(
                                    "📙 Unknown command from user #{}. Command text: {text}",
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

    pub async fn callback_handler(
        bot: Bot,
        q: CallbackQuery,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let chat_id: ChatId = q.message.clone().unwrap().chat().id;
        let chat_id_i64: i64 = chat_id.clone().to_string().parse::<i64>().unwrap();
        let action = q.data;
        let user_lang_map: HashMap<&str, &str> = get_lang_map(chat_id_i64).await;

        if !check_user_rec_avail(chat_id_i64).await {
            return Ok(());
        }

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
                    gen_and_send_main_menu(&bot, chat_id, chat_id_i64, user_lang_map).await;
                }
            }
            Some(act) if act == "custom_charset".to_string() => {
                if rules.custom_charset == "" {
                    set_user_dialog_context(chat_id_i64, "custom_charset").await;
                    bot.send_message(chat_id.clone(), user_lang_map["dialog_ent_cch"])
                        .parse_mode("HTML".parse().unwrap())
                        .await?;
                } else {
                    rules.custom_charset = "".to_string();
                    rules.reconfigure_rules_according_selector(act.clone());
                    let set_rules_res = update_rules(chat_id_i64, rules).await;
                    if set_rules_res {
                        remove_prev_mess(&bot, chat_id, chat_id_i64, "last_gen_mess_id").await;
                        remove_prev_mess(&bot, chat_id, chat_id_i64, "last_menu_mess_id").await;
                        gen_and_send_main_menu(&bot, chat_id, chat_id_i64, user_lang_map).await;
                    }
                }
            }
            Some(act) if act == "pwd_len".to_string() => {
                set_user_dialog_context(chat_id_i64, "pwd_len").await;
                bot.send_message(chat_id.clone(), user_lang_map["dialog_ent_plen"])
                    .parse_mode("HTML".parse().unwrap())
                    .await?;
            }
            Some(act) if act == "pwd_quantity".to_string() => {
                set_user_dialog_context(chat_id_i64, "pwd_quantity").await;
                bot.send_message(chat_id.clone(), user_lang_map["dialog_ent_pqua"])
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

                let mess_result = if rules.pwd_quantity == 1 {
                    let pwd = pgen_from_rules.generate(rules.pwd_len as u32);
                    let mess = bot
                        .send_message(
                            chat_id.clone(),
                            format!(
                                "<i>{} </i><b><code>{pwd}</code></b>",
                                user_lang_map["dialog_pwd_is"]
                            ),
                        )
                        .parse_mode("HTML".parse().unwrap())
                        .reply_markup(
                            KeyboardMarkup::new([[KeyboardButton::new("🧹 pwd")]])
                                .one_time_keyboard()
                                .resize_keyboard(),
                        )
                        .await?;
                    let now_str = get_now_str();
                    println!("[{now_str}] 🎲 - New password for user #{chat_id_i64} was sent.");
                    info!("🎲 New password for user #{chat_id_i64} was sent.");
                    mess
                } else {
                    let pwds: Vec<String> = (0..(rules.pwd_quantity - 1))
                        .map(|_| pgen_from_rules.generate(rules.pwd_len as u32))
                        .collect();
                    let mess = bot
                        .send_message(
                            chat_id.clone(),
                            format!(
                                "<i>{} </i><b><code>{:#?}</code></b>",
                                user_lang_map["dialog_pwd_is"], pwds
                            ),
                        )
                        .parse_mode("HTML".parse().unwrap())
                        .reply_markup(
                            KeyboardMarkup::new([[KeyboardButton::new("🧹 pwd")]])
                                .one_time_keyboard()
                                .resize_keyboard(),
                        )
                        .await?;
                    mess
                };

                let mess_id: i32 = mess_result.id.to_string().parse().unwrap();
                increase_user_gen_count(chat_id_i64).await;
                set_last_mess_id(chat_id_i64, mess_id, "last_gen_mess_id").await;
            }
            _ => {
                warn!("📙 Unrecognized callback action!");
            }
        }
        Ok(())
    }

    async fn remove_prev_mess(bot: &Bot, chat_id: ChatId, chat_id_i64: i64, id_field: &str) {
        let prev_mess_id = get_last_mess_id(chat_id_i64, id_field).await;
        match prev_mess_id {
            Some(mess_id) => {
                let del_res = bot.delete_message(chat_id, MessageId(mess_id)).await;
                match del_res {
                    Ok(_) => debug!(
                    "📗 Mess #{mess_id} at field {id_field} of user:{} successfully removed.",
                    chat_id.clone().to_string()
                ),
                    Err(_) => debug!(
                    "📙 Mess #{mess_id} at field {id_field} of user:{} was not found or already has been removed!",
                    chat_id.clone().to_string()
                ),
                }
            }
            None => debug!("📙 No records to remove \"{id_field}\" found for user #{chat_id_i64}."),
        }
    }

    async fn change_iface_lang(bot: Bot, chat_id: ChatId, chat_id_i64: i64, lang_id: &str) {
        debug!("📗 User #{chat_id_i64} enter command /{lang_id}.");

        if check_user_rec_avail(chat_id_i64).await {
            set_user_app_lang(chat_id_i64, lang_id).await;
            remove_prev_mess(&bot, chat_id, chat_id_i64, "last_gen_mess_id").await;
            remove_prev_mess(&bot, chat_id, chat_id_i64, "last_menu_mess_id").await;

            let user_lang_map: HashMap<&str, &str> = get_lang_map(chat_id_i64).await;
            gen_and_send_main_menu(&bot, chat_id, chat_id_i64, user_lang_map).await;
            set_user_dialog_context(chat_id_i64, "NULL").await;
        }
    }
}
