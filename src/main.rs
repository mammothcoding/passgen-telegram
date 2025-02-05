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

use crate::engine::db_processing::db_processing::init as db_pool_init;
use crate::engine::env_processing::env_processing::DotEnv;
use crate::engine::log::log::init as log_init;
use crate::engine::tg_processing::tg_processing::start_tgbot_service;
use crate::engine::web_stat::web_stat::start_stat_service;
use crate::structs::rules::rules::Rules;
use clap::{arg, Parser};
use clap_derive::Parser as ClapParser;
use log::error;
use std::process::Command;
use std::{env, process};
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
            start_tgbot_service(env_data, args).await;
        }
        "stat" => {
            if !&env_data.web_stat_socket_addr.is_empty() && !&env_data.web_stat_addrs.is_empty() {
                start_stat_service(env_data).await;
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
