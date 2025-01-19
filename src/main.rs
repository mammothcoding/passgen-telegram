mod engine {
    pub mod db_processing;
    pub mod env_processing;
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
use crate::engine::tg_processing::tg_processing::{
    callback_handler, message_handler, send_test_tg_mess,
};
use crate::engine::web_stat::web_stat::get_router;
use crate::structs::rules::rules::Rules;
use ::log::info;
use std::future::IntoFuture;
use teloxide::Bot;
use teloxide::{prelude::*, update_listeners::webhooks};

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

    if !&env_data.webstat_socket_addr.is_empty() && !&env_data.web_stat_addrs.is_empty() {
        // Web_stat
        let now_str = get_now_str();
        let web_stat_router = get_router();

        let web_stat_listener = tokio::net::TcpListener::bind(&env_data.webstat_socket_addr)
            .await
            .expect(&format!(
                "[{now_str}] 🚫 Error on init listener for web_stat_server!"
            ));
        let web_stat_serv = axum::serve(web_stat_listener, web_stat_router);
        println!("[{now_str}] ✅ - Web_stat_server prep OK.");
        info!("✅ Web_stat_server prep OK.");

        // Setup teloxide listener.
        let now_str = get_now_str();
        let bot: Bot = Bot::new(&env_data.tg_bot_token);
        let listener = webhooks::axum(
            bot.clone(),
            webhooks::Options::new(
                (&env_data.tg_bot_socket_addr).parse().expect(&format!(
                    "[{now_str}] 🚫 - Incorrect TELEGRAM_BOT_SOCKET_ADDR in the .env file!"
                )),
                (&env_data.tg_web_hook_url).parse().expect(&format!(
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

        let mut telox_binding = Dispatcher::builder(bot.clone(), telox_handler)
            .dependencies(dptree::deps![env_data])
            //.enable_ctrlc_handler()
            .build();
        let telox_disp = telox_binding.dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text(&format!(
                "[{now_str}] 🚫 - An error from the update listener"
            )),
        );

        let (_error1, _error2) =
            tokio::join!(web_stat_serv.into_future(), telox_disp.into_future());
    } else {
        // Setup teloxide listener.
        let now_str = get_now_str();
        let bot: Bot = Bot::new(&env_data.tg_bot_token);
        let listener = webhooks::axum(
            bot.clone(),
            webhooks::Options::new(
                (&env_data.tg_bot_socket_addr).parse().expect(&format!(
                    "[{now_str}] 🚫 - Incorrect TELEGRAM_BOT_SOCKET_ADDR in the .env file!"
                )),
                (&env_data.tg_web_hook_url).parse().expect(&format!(
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
            .dependencies(dptree::deps![env_data])
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
}
