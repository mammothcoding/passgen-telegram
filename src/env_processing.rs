pub mod env_processing {
    use crate::get_now_str;
    use env_file_reader::read_file;
    use log::{error, info};
    use std::collections::HashMap;
    use std::{env, process};

    #[derive(Clone)]
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
        pub webstat_socket_addr: String,
        pub tg_users_id_to_web_stat_access: Vec<i64>,
        pub web_stat_addrs: Vec<[String; 2]>,
    }

    impl DotEnv {
        // Obtain variables from ".env" file stored near binary file.
        pub fn parse_dot_env() -> DotEnv {
            let default_db_port = Option::from("5432");
            let default_db_name = Option::from("tg_passgen_db");
            let default_log_stderr_lvl = Option::from("off");
            let default_log_logfile_lvl = Option::from("off");
            let default_log_trigger_file_size = Option::from("1048576");
            let default_log_files_count = Option::from("5");
            let default_log_files_path = Option::from("log");
            let default_webstat_socket_addr = Option::from("");
            let default_tg_user_id_to_web_stat_access = Option::from("");
            let default_web_stat_addrs = Option::from("");

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
                default_val: Option<&str>,
                missing_keys: &mut Vec<&'a str>,
            ) -> String {
                match default_val {
                    Some(default_val) => {
                        if env_variables.contains_key(key) {
                            let res = env_variables[key].to_string();
                            if res.is_empty() {
                                default_val.to_string()
                            } else {
                                res
                            }
                        } else {
                            default_val.to_string()
                        }
                    }
                    _ => {
                        if env_variables.contains_key(key) {
                            let res = env_variables[key].to_string();
                            if res.is_empty() {
                                missing_keys.push(key);
                                "".to_string()
                            } else {
                                res
                            }
                        } else {
                            missing_keys.push(key);
                            "".to_string()
                        }
                    }
                }
            }

            let res = DotEnv {
                tg_bot_token: get_env_var(
                    "TELEGRAM_BOT_TOKEN",
                    &env_variables,
                    None,
                    &mut missing_keys,
                ),
                tg_web_hook_url: get_env_var(
                    "TELEGRAM_WEBHOOK_URL",
                    &env_variables,
                    None,
                    &mut missing_keys,
                ),
                tg_bot_socket_addr: get_env_var(
                    "TELEGRAM_BOT_SOCKET_ADDR",
                    &env_variables,
                    None,
                    &mut missing_keys,
                ),
                db_host: get_env_var("DB_HOST", &env_variables, None, &mut missing_keys),
                db_port: {
                    match get_env_var(
                        "DB_PORT",
                        &env_variables,
                        default_db_port,
                        &mut missing_keys,
                    )
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
                db_username: get_env_var("DB_USERNAME", &env_variables, None, &mut missing_keys),
                db_password: get_env_var("DB_PASSWORD", &env_variables, None, &mut missing_keys),
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
                    match get_env_var(
                        "LOG_TRIGGER_FILE_SIZE",
                        &env_variables,
                        default_log_trigger_file_size,
                        &mut missing_keys,
                    )
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
                    match get_env_var(
                        "LOG_FILES_COUNT",
                        &env_variables,
                        default_log_files_count,
                        &mut missing_keys,
                    )
                    .parse::<u32>()
                    {
                        Ok(count) => count,
                        Err(_) => {
                            missing_keys.push("LOG_FILES_COUNT");
                            0
                        }
                    }
                },
                log_files_path: get_env_var(
                    "LOG_FILES_PATH",
                    &env_variables,
                    default_log_files_path,
                    &mut missing_keys,
                ),
                webstat_socket_addr: get_env_var(
                    "WEB_STAT_SOCKET_ADDR",
                    &env_variables,
                    default_webstat_socket_addr,
                    &mut missing_keys,
                ),
                tg_users_id_to_web_stat_access: {
                    let mut users_ids: Vec<i64> = Vec::new();
                    let from_env = get_env_var(
                        "TG_USERS_ID_TO_WEB_STAT_ACCESS",
                        &env_variables,
                        default_tg_user_id_to_web_stat_access,
                        &mut missing_keys,
                    );
                    if from_env.is_empty() {
                        users_ids
                    } else {
                        let potential_ids = &from_env[..].split(',').collect::<Vec<&str>>();
                        for potential_id in potential_ids {
                            match potential_id.trim().parse::<i64>() {
                                Ok(id) => users_ids.push(id),
                                Err(_) => {
                                    missing_keys.push("TG_USERS_ID_TO_WEB_STAT_ACCESS");
                                    break;
                                }
                            }
                        }
                        users_ids
                    }
                },
                web_stat_addrs: {
                    let mut res_pairs: Vec<[String; 2]> = Vec::new();
                    let from_env = get_env_var(
                        "WEB_STAT_ADDRS",
                        &env_variables,
                        default_web_stat_addrs,
                        &mut missing_keys,
                    );
                    if from_env.is_empty() {
                        res_pairs
                    } else {
                        let potential_pairs = &from_env[..].split(',').collect::<Vec<&str>>();
                        for potential_pair in potential_pairs {
                            if potential_pair.contains('|') {
                                let pair = potential_pair.trim().split('|').collect::<Vec<&str>>();
                                if pair.len() == 2 && !pair[0].is_empty() && !pair[1].is_empty() {
                                    res_pairs.push([pair[0].to_string(), pair[1].to_string()]);
                                } else {
                                    missing_keys.push("WEB_STAT_ADDRS");
                                    break;
                                }
                            } else {
                                missing_keys.push("WEB_STAT_ADDRS");
                                break;
                            }
                        }
                        res_pairs
                    }
                },
            };

            let now_str = get_now_str();
            if missing_keys.is_empty() {
                println!("[{now_str}] ✅ - .env init is OK.");
                info!("✅ - .env init is OK.");
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
