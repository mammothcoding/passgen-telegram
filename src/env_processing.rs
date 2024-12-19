pub mod env_processing {
    use crate::get_now_str;
    use env_file_reader::read_file;
    use std::collections::HashMap;
    use std::{env, process};

    pub struct DotEnv {
        pub tg_bot_token: String,
        pub tg_web_hook_url: String,
        pub tg_bot_socket_addr: String,
        pub db_host: String,
        pub db_port: String,
        pub db_name: String,
        pub db_username: String,
        pub db_password: String,
    }

    impl DotEnv {
        // Get variables from ".env" file stored near binary file.
        pub fn parse_dot_env() -> DotEnv {
            let cur_exe_binding = env::current_exe().unwrap();
            let env_path = &cur_exe_binding.with_file_name(".env");
            let env_variables = read_file(env_path).expect("🚫 Could not load .env file!");
            let mut missing_keys: Vec<&str> = Vec::new();

            fn get_env_var<'a>(
                key: &'a str,
                env_variables: &HashMap<String, String>,
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
                    missing_keys.push(key);
                    "".to_string()
                }
            }

            let res = DotEnv {
                tg_bot_token: get_env_var("TELEGRAM_BOT_TOKEN", &env_variables, &mut missing_keys),
                tg_web_hook_url: get_env_var(
                    "TELEGRAM_WEBHOOK_URL",
                    &env_variables,
                    &mut missing_keys,
                ),
                tg_bot_socket_addr: get_env_var(
                    "TELEGRAM_BOT_SOCKET_ADDR",
                    &env_variables,
                    &mut missing_keys,
                ),
                db_host: get_env_var("DB_HOST", &env_variables, &mut missing_keys),
                db_port: get_env_var("DB_PORT", &env_variables, &mut missing_keys),
                db_name: get_env_var("DB_NAME", &env_variables, &mut missing_keys),
                db_username: get_env_var("DB_USERNAME", &env_variables, &mut missing_keys),
                db_password: get_env_var("DB_PASSWORD", &env_variables, &mut missing_keys),
            };

            let now_str = get_now_str();
            if missing_keys.is_empty() {
                println!("✅ [{now_str}] .env is OK.");
                res
            } else {
                println!("🚫 [{now_str}] Missing or empty .env keys:");
                println!("{:?}", missing_keys);
                process::exit(1);
            }
        }
    }
}
