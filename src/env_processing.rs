pub mod env_processing {
    use crate::get_now_str;
    use env_file_reader::read_file;
    use std::collections::HashMap;
    use std::{env, process};
    use log::{error, info};

    pub struct DotEnv {
        pub tg_bot_token: String,
        pub tg_web_hook_url: String,
        pub tg_bot_socket_addr: String,
        pub db_host: String,
        pub db_port: u32,
        pub db_name: String,
        pub db_username: String,
        pub db_password: String,
        pub log_stderr_lvl: String,
        pub log_logfile_lvl: String,
        pub log_trigger_file_size: u64,
        pub log_files_count: u32,
        pub log_files_path: String,
    }

    impl DotEnv {
        // Obtain variables from ".env" file stored near binary file.
        pub fn parse_dot_env() -> DotEnv {
            let default_db_port = "5432";
            let default_db_name = "tg_passgen_db";
            let default_log_stderr_lvl = "off";
            let default_log_logfile_lvl = "off";
            let default_log_trigger_file_size = "1048576";
            let default_log_files_count = "5";
            let default_log_files_path = "log";

            let now_str = get_now_str();
            let cur_exe_binding = env::current_exe()
                .expect(&format!("[{now_str}] 🚫 - Get cur_exe_binding path error!"));
            let env_path = &cur_exe_binding.with_file_name(".env");
            let env_variables: HashMap<String, String> =
                read_file(env_path).expect(&format!("[{now_str}] 🚫 - Could not load .env file!"));
            let mut missing_keys: Vec<&str> = Vec::new();

            fn get_env_var<'a>(
                key: &'a str,
                env_variables: &HashMap<String, String>,
                default_val: &str,
                missing_keys: &mut Vec<&'a str>,
            ) -> String {
                if env_variables.contains_key(key) {
                    let res = env_variables[key].to_string();
                    if res.is_empty() {
                        missing_keys.push(key);
                        "".to_string()
                    } else {
                        res
                    }
                } else {
                    if default_val.is_empty() {
                        missing_keys.push(key);
                        "".to_string()
                    } else {
                        default_val.to_string()
                    }
                }
            }

            let res = DotEnv {
                tg_bot_token: get_env_var(
                    "TELEGRAM_BOT_TOKEN",
                    &env_variables,
                    "",
                    &mut missing_keys,
                ),
                tg_web_hook_url: get_env_var(
                    "TELEGRAM_WEBHOOK_URL",
                    &env_variables,
                    "",
                    &mut missing_keys,
                ),
                tg_bot_socket_addr: get_env_var(
                    "TELEGRAM_BOT_SOCKET_ADDR",
                    &env_variables,
                    "",
                    &mut missing_keys,
                ),
                db_host: get_env_var("DB_HOST", &env_variables, "", &mut missing_keys),
                db_port: {
                    match get_env_var("DB_PORT", &env_variables, default_db_port, &mut missing_keys)
                        .parse::<u32>()
                    {
                        Ok(port) => port,
                        Err(_) => {
                            missing_keys.push("DB_PORT");
                            0
                        }
                    }
                },
                db_name: get_env_var(
                    "DB_NAME",
                    &env_variables,
                    default_db_name,
                    &mut missing_keys,
                ),
                db_username: get_env_var("DB_USERNAME", &env_variables, "", &mut missing_keys),
                db_password: get_env_var("DB_PASSWORD", &env_variables, "", &mut missing_keys),
                log_stderr_lvl: get_env_var(
                    "LOG_STDERR_LVL",
                    &env_variables,
                    default_log_stderr_lvl,
                    &mut missing_keys,
                ),
                log_logfile_lvl: get_env_var(
                    "LOG_LOGFILE_LVL",
                    &env_variables,
                    default_log_logfile_lvl,
                    &mut missing_keys,
                ),
                log_trigger_file_size: {
                    match get_env_var("LOG_TRIGGER_FILE_SIZE", &env_variables, default_log_trigger_file_size, &mut missing_keys)
                        .parse::<u64>()
                    {
                        Ok(size) => size,
                        Err(_) => {
                            missing_keys.push("LOG_TRIGGER_FILE_SIZE");
                            0
                        }
                    }
                },
                log_files_count: {
                    match get_env_var("LOG_FILES_COUNT", &env_variables, default_log_files_count, &mut missing_keys)
                        .parse::<u32>()
                    {
                        Ok(count) => count,
                        Err(_) => {
                            missing_keys.push("LOG_FILES_COUNT");
                            0
                        }
                    }
                },
                log_files_path: get_env_var("LOG_FILES_PATH", &env_variables, default_log_files_path, &mut missing_keys),
            };

            let now_str = get_now_str();
            if missing_keys.is_empty() {
                println!("[{now_str}] ✅ - .env init is OK");
                info!("✅ - .env init is OK");
                res
            } else {
                println!("[{now_str}] 🚫 - Missing .env keys:");
                println!("{:#?}", missing_keys);
                error!("[{now_str}] 🚫 - Missing .env keys:");
                error!("{:#?}", missing_keys);
                process::exit(1);
            }
        }
    }
}
