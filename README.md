![alt text](./pics/McDev_thin_900x70.png "McDev_thin_900x70.png")

[![Latest version](https://img.shields.io/crates/v/passgen-telegram.svg)](https://crates.io/crates/passgen-telegram)
[![Download](https://img.shields.io/crates/d/passgen-telegram.svg)](https://crates.io/crates/passgen-telegram)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://choosealicense.com/licenses/mit/)
[![Build Status](https://github.com/mammothcoding/passgen-telegram/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/mammothcoding/passgen-telegram/actions/workflows/rust.yml)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
![Endpoint Badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fstat.tg.passgen.mamont.xyz%2Fbots-users-count)
![Endpoint Badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fstat.tg.passgen.mamont.xyz%2Ftotal-count-generated-pwds)

Readme in different languages:
[EN](https://github.com/mammothcoding/passgen-telegram/blob/master/README.md)
[RU](https://github.com/mammothcoding/passgen-telegram/blob/master/README.ru.md)

# 📲 Passgen-telegram
Telegram bot-service for generating cryptographically secure passwords/tokens and other sets and sequences.

![alt text](./pics/passgen-telegram_demo.gif "passgen-telegram_demo.gif")

The service can serve multiple generator bots with corresponding telegram tokens, names, etc.

### Telegram-bot
Work with Telegram is implemented using the ["teloxide"](https://github.com/teloxide/teloxide) library.

A separate process is automatically created for each bot, listening on its own port according to the configuration settings.

![alt text](./pics/service_catalogue.png "service_catalogue.png")

The main service configuration settings must be in the same directory as the service startup file in .env.

### Env
Work with .env files is implemented using the ["env-file-reader"](https://github.com/jofas/env_file_reader) library.
An example .env file in the ["examples"](./examples/) directory of this repository.

### Passgen
Passwords are generated with the help of our own library ["passgen-lib"](https://github.com/mammothcoding/passgen-lib).

### DB
The ["sqlx"](https://github.com/launchbadge/sqlx) library is used to store conditions for generating user passwords, context information for working with bots and information for statistics

DB storage configured for Postgres databases.

### Statistics web-service

![alt text](./pics/web_stat_screen.png "web_stat_screen.png")

For the collection and display of statistics about the bot's work, a web-based statistics service is implemented with the help of the ["axum"](https://github.com/tokio-rs/axum) library.

The statistics web service is automatically started in a separate process and is available via links in the bot interface to users specified in .env.
The web service settings are also set in the .env.

### Log & stdout

![alt text](./pics/log_stdout.png "log_stdout.png")

The ["log4rs"](https://github.com/estk/log4rs) library is used to provide logging and output information about the processes of services operation.
Logging and output are configured in .env.

### ps:

This project was also created with the purpose of studying the capabilities of the 🦀Rust language. Therefore, some solutions in the code are not optimal.


### License

[MIT](https://choosealicense.com/licenses/mit/)

### Our other passgen projects:
[passgen-lib](https://github.com/mammothcoding/passgen-lib)

[passgen-desktop](https://github.com/mammothcoding/passgen-desktop)

[passgen-console-linuxwin](https://github.com/mammothcoding/passgen-console-linuxwin)

[passgen-cmd](https://github.com/mammothcoding/passgen-cmd)
