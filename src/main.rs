mod db_processing;
pub mod env_processing;
mod lang_processing;
mod log;
mod rules;
mod tg_processing;
mod user;
mod web_stat;

use crate::db_processing::db_processing::init as db_pool_init;
use crate::env_processing::env_processing::DotEnv;
use crate::log::log::init as log_init;
use crate::tg_processing::tg_processing::{callback_handler, message_handler, send_test_tg_mess};
use crate::web_stat::web_stat::{gen_token, get_web_state_token};
use ::log::info;
use axum::{routing::get, Router};
use rules::rules::Rules;
use std::future::IntoFuture;
use std::net::SocketAddr;
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

    // Web_stat
    let now_str = get_now_str();
    let web_stat_app = Router::new()
        .route("/", get(|| web_stat_handler()));
        //.route("/favicon.ico", get(|| axum::response:: ::NamedFile::open("/path/to/your/image/file/../favicon.png"));
    let web_stat_adr = SocketAddr::from(([0, 0, 0, 0], 8001));
    let web_stat_listener = tokio::net::TcpListener::bind(web_stat_adr)
        .await
        .expect(&format!(
            "[{now_str}] 🚫 Error on init listener for web_stat_server!"
        ));
    let web_stat_serv = axum::serve(web_stat_listener, web_stat_app);
    println!("[{now_str}] ✅ - Web_stat_server prep OK.");
    info!("✅ Web_stat_server prep OK.");

    // Setup teloxide listener.
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

    let (_error1, _error2) = tokio::join!(web_stat_serv.into_future(), telox_disp.into_future());
    println!("[{now_str}] 🚫 - Errors on par_run!");
}

async fn web_stat_handler() -> String {
    gen_token();
    get_web_state_token().to_string()
    //"Hello World! web_stat_handler".to_string()
}
