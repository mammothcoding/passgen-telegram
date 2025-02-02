mod engine {
    pub mod db_processing;
    pub mod env_processing;
    pub mod glob_state;
    pub mod lang_processing;
    pub mod log;
    pub mod tg_processing;
    pub mod web_stat;
}
mod structs {
    pub mod rules;
    pub mod user;
}

use crate::engine::db_processing::db_processing::{
    get_total_pwds_sum, get_users_count, init as db_pool_init,
};
use crate::engine::env_processing::env_processing::DotEnv;
use crate::engine::glob_state::glob_state::{
    get_bot_name, set_bot_name, set_bots_gen_pwds as glob_st_set_bots_gen_pwds,
    set_bots_users_count as glob_st_set_bots_users_count,
};
use crate::engine::log::log::init as log_init;
use crate::engine::tg_processing::tg_processing::{
    callback_handler, message_handler, send_test_tg_mess,
};
use crate::engine::web_stat::web_stat::get_router;
use crate::structs::rules::rules::Rules;
use ::log::info;
use clap::{arg, Parser};
use clap_derive::Parser as ClapParser;
use log::{debug, error};
use std::future::IntoFuture;
use std::process::Command;
use std::time::Duration;
use std::{env, process};
use teloxide::Bot;
use teloxide::{prelude::*, update_listeners::webhooks};
use tokio::signal;

#[derive(ClapParser, Debug)]
#[command(
    version,
    verbatim_doc_comment,
    about = "\n\n⚙️ Passgen-telegram server core."
)]
struct Args {
    /// 🚀 Servicename to run.
    /// Variants:
    /// bot         - start the bot service (In addition, it will be necessary to pass the bot id);
    /// stat        - start the bots statistics web-service.
    #[arg(short, long, default_value = "", verbatim_doc_comment)]
    service: String,

    /// Telegram bot id (123456789).
    #[arg(short, long, default_value = "", verbatim_doc_comment)]
    id: String,
}

pub fn get_now_str() -> String {
    chrono::Local::now()
        .format("%d-%b-%y %X%.6f %Z")
        .to_string()
}

#[tokio::main]
async fn main() {
    // Env
    let env_data = DotEnv::parse_dot_env();

    // Log
    log_init(&env_data);

    // DB
    db_pool_init(&env_data).await;

    // Run
    let args: Args = Args::parse();
    let service = args.service.as_str().as_ref();

    match service {
        "bot" => {
            let bot_id: String = if !args.id.is_empty() {
                args.id.clone()
            } else {
                let now_str = get_now_str();
                println!("[{now_str}] 🚫 - Could not start the bot service. The required bot id is not passed!");
                error!("🚫 - Could not start the bot service. The required bot id is not passed!");
                process::exit(1);
            };

            let mut tg_bot_identifiers: Vec<String> = Vec::new();
            for identifier_arr in &env_data.tg_bots_identifiers {
                if bot_id == identifier_arr[0].split(':').collect::<Vec<&str>>()[0].to_string() {
                    tg_bot_identifiers = identifier_arr.clone().into_iter().collect();

                    match &env_data.web_stat_bots_usernames.get(&args.id[..]) {
                        Some(bot_name) => set_bot_name(&bot_name[..].to_string()),
                        _ => (),
                    }
                    info!("Init bot_name is {}.", get_bot_name());
                }
            }
            if tg_bot_identifiers.is_empty() {
                let now_str = get_now_str();
                println!("[{now_str}] 🚫 - Could not start the bot service. The required bot id is not passed!");
                error!("🚫 - Could not start the bot service. The required bot id is not passed!");
                process::exit(1);
            }

            // Setup teloxide listener.
            let now_str = get_now_str();
            let bot: Bot = Bot::new(&tg_bot_identifiers[0]);
            let listener = webhooks::axum(
                bot.clone(),
                webhooks::Options::new(
                    (&tg_bot_identifiers[2]).parse().expect(&format!(
                        "[{now_str}] 🚫 - Incorrect TELEGRAM_BOT_SOCKET_ADDR in the .env file!"
                    )),
                    (&tg_bot_identifiers[1]).parse().expect(&format!(
                        "[{now_str}] 🚫 - Incorrect TELEGRAM_WEBHOOK_URL in the .env file!"
                    )),
                ),
            )
            .await
            .expect(&format!("[{now_str}] 🚫 - Couldn't setup webhook"));

            let telox_handler = dptree::entry()
                .branch(Update::filter_message().endpoint(message_handler))
                .branch(Update::filter_callback_query().endpoint(callback_handler));

            send_test_tg_mess(&bot).await;

            Dispatcher::builder(bot.clone(), telox_handler)
                .dependencies(dptree::deps![env_data.clone()])
                //.enable_ctrlc_handler()
                .build()
                .dispatch_with_listener(
                    listener,
                    LoggingErrorHandler::with_custom_text(&format!(
                        "[{now_str}] 🚫 - An error from the update listener"
                    )),
                )
                .await;
        }
        "stat" => {
            if !&env_data.web_stat_socket_addr.is_empty() && !&env_data.web_stat_addrs.is_empty() {
                let now_str = get_now_str();
                let web_stat_router = get_router(&env_data);

                let web_stat_listener =
                    tokio::net::TcpListener::bind(&env_data.web_stat_socket_addr)
                        .await
                        .expect(&format!(
                            "[{now_str}] 🚫 Error on init listener for web_stat_server!"
                        ));
                let web_stat_serv = axum::serve(web_stat_listener, web_stat_router);
                println!("[{now_str}] ✅ - Web_stat_server prep OK.");
                info!("✅ Web_stat_server prep OK.");

                async fn loop_for_updating_statistical_glob_vars(timeout: &u64) {
                    loop {
                        let users_count: i32 = get_users_count().await;
                        let passwords_count = get_total_pwds_sum().await;

                        glob_st_set_bots_users_count(users_count);
                        glob_st_set_bots_gen_pwds(passwords_count);

                        debug!("📗 Statistical global vars was updated.");
                        tokio::time::sleep(Duration::from_secs(*timeout)).await;
                    }
                }

                let (_err1, _err2) = tokio::join!(
                    web_stat_serv.into_future(),
                    loop_for_updating_statistical_glob_vars(
                        &env_data.statistical_glob_vars_update_timeout
                    )
                    .into_future()
                );
            } else {
                let now_str = get_now_str();
                println!("[{now_str}] 🚫 - Could not start the bots statistics web-service. The required .env keys are empty!");
                error!("🚫 - Could not start the bots statistics web-service. The required .env keys are empty!");
                process::exit(1);
            }
        }
        _ => {
            let now_str = get_now_str();
            let service_file_name = env!("CARGO_PKG_NAME");
            let cur_exe_binding = env::current_exe()
                .expect(&format!("[{now_str}] 🚫 - Get cur_exe_binding path error!"));
            let service_full_path = &cur_exe_binding.with_file_name(service_file_name);

            // Start bot-service processes.
            for identifier_arr in &env_data.tg_bots_identifiers {
                let bot_id = identifier_arr[0].split(':').collect::<Vec<&str>>()[0].to_string();
                Command::new(service_full_path)
                    .args(["-s", "bot", "-i", &bot_id[..]])
                    .spawn()
                    .expect(&format!("[{now_str}] 🚫 - Failed to execute bot-service process for bot_id: #{bot_id}!"));
            }

            // Start web_stat-service process.
            Command::new(service_full_path)
                .args(["-s", "stat"])
                .spawn()
                .expect(&format!(
                    "[{now_str}] 🚫 - Failed to execute web_stat-service process!"
                ));

            let ctrlc = signal::ctrl_c().await;
            match ctrlc {
                Ok(_) => println!("Kill all child processes. Bye! ⏻"),
                Err(_err) => println!("Error on ctrl-c received! {}", _err),
            }
        }
    }
}
