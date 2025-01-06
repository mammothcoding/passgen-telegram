mod db_processing;
pub mod env_processing;
mod lang_processing;
mod log;
mod rules;
mod tg_processing;
mod user;

use crate::db_processing::db_processing::init as db_pool_init;
use crate::env_processing::env_processing::DotEnv;
use crate::log::log::init as log_init;
use crate::tg_processing::tg_processing::{callback_handler, message_handler, send_test_tg_mess};
use rules::rules::Rules;
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

    // Setup listener.
    let now_str = get_now_str();
    let bot: Bot = Bot::new(&env_data.tg_bot_token);
    let listener = webhooks::axum(
        bot.clone(),
        webhooks::Options::new(
            (&env_data.tg_bot_socket_addr).parse().unwrap(),
            (&env_data.tg_web_hook_url).parse().unwrap(),
        ),
    )
    .await
    .expect(&format!("[{now_str}] 🚫 - Couldn't setup webhook"));

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler));

    send_test_tg_mess(&bot).await;

    Dispatcher::builder(bot.clone(), handler)
        //.dependencies(dptree::deps![db_pool])
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text(&format!(
                "[{now_str}] 🚫 - An error from the update listener"
            )),
        )
        .await;
}
