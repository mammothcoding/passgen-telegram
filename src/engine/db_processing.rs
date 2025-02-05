pub mod db_processing {
    use crate::engine::env_processing::env_processing::DotEnv;
    use crate::engine::glob_state::glob_state::get_bot_name;
    use crate::engine::lang_processing::lang_processing::obtain_user_lang_code;
    use crate::engine::web_stat::web_stat::IndexParams;
    use crate::structs::user::user::User as user_data;
    use crate::{get_now_str, Rules};
    use log::{debug, error, info, warn};
    use sqlx::postgres::PgRow;
    use sqlx::types::Json;
    use sqlx::{Connection, Executor, PgConnection, PgPool, Pool, Postgres, QueryBuilder, Row};
    use std::process;
    use teloxide_core::types::User;
    use tokio::sync::OnceCell;

    static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();

    pub async fn init(env_data: &DotEnv) {
        let connection = PgConnection::connect(
            format!(
                "postgres://{}:{}@{}:{}",
                env_data.db_username, env_data.db_password, env_data.db_host, env_data.db_port
            )
            .as_str(),
        )
        .await;
        if let Err(_err) = &connection {
            let now_str = get_now_str();
            println!(
                "[{now_str}] 🚫 - Error on connection to Postgres: '{}'.",
                _err
            );
            error!("Error on connection to Postgres: '{}'.", _err);
            process::exit(1);
        }

        let check_or_create_db_res = connection
            .unwrap()
            .execute(format!(r#"CREATE DATABASE "{}";"#, env_data.db_name).as_str())
            .await;
        match check_or_create_db_res {
            Ok(_) => debug!(
                "DB '{}' doesn't exist. Create new DB is successfully.",
                env_data.db_name
            ),
            Err(err) => {
                let now_str = get_now_str();
                if err.to_string().contains("already exists") {
                    debug!("DB '{}' is already exists.", env_data.db_name);
                } else {
                    println!("[{now_str}] 🚫 - Error check_or_create_db: '{}'.", err);
                    error!("Error check_or_create_db: '{}'.", err);
                    process::exit(1);
                }
            }
        }

        let connection_pool = PgPool::connect(
            format!(
                "postgres://{}:{}@{}:{}/{}",
                env_data.db_username,
                env_data.db_password,
                env_data.db_host,
                env_data.db_port,
                env_data.db_name
            )
            .as_str(),
        )
        .await;
        if let Err(_err) = &connection_pool {
            let now_str = get_now_str();
            println!("[{now_str}] 🚫 - Error on PgPool::connect: '{}'.", _err);
            error!("Error on PgPool::connect: '{}'.", _err);
            process::exit(1);
        }
        debug!("Connected to '{}' DB.", env_data.db_name);

        let pool = connection_pool.unwrap();
        let migr_res = sqlx::migrate!("./migrations").run(&pool).await;
        let now_str = get_now_str();
        if let Err(_err) = migr_res {
            println!("[{now_str}] 🚫 - Migration error: '{}'.", _err);
            error!("Migration error: '{}'.", _err);
            process::exit(1);
        }
        debug!("DB migrations OK.");

        DB_POOL.get_or_init(|| async { pool }).await;

        let now_str = get_now_str();
        println!("[{now_str}] ✅ - DB init is OK.");
        info!("✅ DB init is OK.");
    }

    pub async fn check_user_rec_avail(chat_id: i64, bot_id: i64) -> bool {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT id from main WHERE chat_id = ");
        q.push_bind(chat_id);
        q.push(" AND bot_id = ");
        q.push_bind(bot_id);
        q.push(" LIMIT 1");
        let res = q.build().fetch_one(pool);

        match res.await {
            Ok(_ok) => {
                debug!(
                    "📗 Query check_user_rec_avail is successfully completed: {:?}.",
                    _ok
                );
                true
            }
            Err(_err) => {
                debug!("📙 Empty result of query check_user_rec_avail: '{_err}'.");
                false
            }
        }
    }

    pub async fn cr_new_user_rec(chat_id: i64, bot_id: i64, from: User) -> bool {
        let user: user_data = user_data {
            username: from.username.unwrap_or("".to_string()),
            first_name: from.first_name,
            last_name: from.last_name.unwrap_or("".to_string()),
            language_code: from.language_code.unwrap_or("".to_string()),
            is_bot: from.is_bot,
        };

        let user_lang_code = obtain_user_lang_code(&user.language_code[..]);

        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> = QueryBuilder::new(
            "INSERT INTO main (
            chat_id, bot_id, user_data, app_lang) VALUES (",
        );
        q.push_bind(chat_id);
        q.push(",");
        q.push_bind(bot_id);
        q.push(",");
        q.push_bind(Json(&user));
        q.push(",");
        q.push_bind(user_lang_code);
        q.push(")");
        let res = q.build().execute(pool).await;

        let now_str = get_now_str();
        match res {
            Ok(_ok) => {
                println!(
                    "[{now_str}] 👤 - New user of {} reg #{chat_id} [{} {:?} {:?}  {:?}].",
                    get_bot_name(),
                    user.first_name,
                    user.last_name,
                    user.username,
                    user.language_code
                );
                info!(
                    "👤 New user of {} reg #{chat_id} [{} {:?} {:?}  {:?}].",
                    get_bot_name(),
                    user.first_name,
                    user.last_name,
                    user.username,
                    user.language_code
                );
                true
            }
            Err(_err) => {
                println!("[{now_str}] 📕 - Error on reg new user #{chat_id} in DB: '{_err}'.");
                error!("📕 Error on reg new user #{chat_id} in DB: '{_err}'.");
                false
            }
        }
    }

    pub async fn get_pgen_rules(chat_id: i64, bot_id: i64) -> Option<Rules> {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT pgen_rules as Json from main WHERE chat_id = ");
        q.push_bind(chat_id);
        q.push(" AND bot_id = ");
        q.push_bind(bot_id);
        q.push(" LIMIT 1");
        let res = q.build().fetch_one(pool).await;

        let now_str = get_now_str();
        match res {
            Ok(_ok) => {
                let pgen_rules_json = &_ok.try_get_raw(0).unwrap().as_str().unwrap()[1..];
                let parse_to_rules = serde_json::from_str::<Rules>(pgen_rules_json);
                match parse_to_rules {
                    Ok(_ok) => {
                        debug!("📗 Get Rules for user #{chat_id} is successfully.");
                        Option::from(_ok)
                    }
                    Err(_err) => {
                        println!("[{now_str}] 📕 - Get pgen_rules & parse to Rules for user #{chat_id} has error: {}", _err);
                        error!(
                            "📕 Get pgen_rules & parse to Rules for user #{chat_id} has error: {}",
                            _err
                        );
                        None
                    }
                }
            }
            Err(_err) => {
                error!("📙 Empty result of query get_pgen_rules: '{_err}'.");
                None
            }
        }
    }

    pub async fn get_last_mess_id(chat_id: i64, bot_id: i64, id_field: &str) -> Option<i32> {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> =
            QueryBuilder::new(format!("SELECT {id_field} from main WHERE chat_id = "));
        q.push_bind(chat_id);
        q.push(" AND bot_id = ");
        q.push_bind(bot_id);
        q.push(" LIMIT 1");
        let res = q.build().fetch_one(pool);

        match res.await {
            Ok(_ok) => {
                debug!(
                    "📗 Query get_last_mess_id is successfully completed: {:?}.",
                    _ok
                );
                _ok.get(0)
            }
            Err(_err) => {
                debug!("📙 Empty result of query get_last_mess_id: '{_err}'.");
                None
            }
        }
    }

    pub async fn set_last_mess_id(chat_id: i64, bot_id: i64, mess_id: i32, id_field: &str) {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> =
            QueryBuilder::new(format!("UPDATE main SET {id_field} = "));
        q.push_bind(mess_id);
        q.push(", updated_at = current_timestamp");
        q.push(" WHERE chat_id = ");
        q.push_bind(chat_id);
        q.push(" AND bot_id = ");
        q.push_bind(bot_id);
        let res = q.build().execute(pool).await;

        match res {
            Ok(_ok) => debug!("📗 Register new mess id #{mess_id} to {id_field} field."),
            Err(_err) => error!("📕 Error on query of register new mess id #{mess_id} to {id_field} field: '{_err}'."),
        }
    }

    pub async fn update_rules(chat_id: i64, bot_id: i64, rules: Rules) -> bool {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE main SET pgen_rules = ");
        q.push_bind(Json(rules));
        q.push(", updated_at = current_timestamp WHERE chat_id = ");
        q.push_bind(chat_id);
        q.push(" AND bot_id = ");
        q.push_bind(bot_id);
        let res = q.build().execute(pool).await;

        let now_str = get_now_str();
        match res {
            Ok(_ok) => {
                debug!("📗 Rules has been updated for user #{chat_id}.");
                true
            }
            Err(_err) => {
                println!(
                    "[{now_str}] 📕 - Error on updating rules for user #{chat_id}.: '{_err}'."
                );
                error!("📕 Error on updating rules for user #{chat_id}.: '{_err}'.");
                false
            }
        }
    }

    pub async fn increase_user_gen_count(chat_id: i64, bot_id: i64, pwd_quantity: u64) {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> =
            QueryBuilder::new("UPDATE main SET gen_count = gen_count + ");
        q.push_bind(pwd_quantity as i64);
        q.push(", updated_at = current_timestamp WHERE chat_id = ");
        q.push_bind(chat_id);
        q.push(" AND bot_id = ");
        q.push_bind(bot_id);
        let res = q.build().execute(pool).await;

        match res {
            Ok(_ok) => debug!("📗 Increase gen_count for user #{chat_id}."),
            Err(_err) => warn!("📕 Error on increase gen_count for user #{chat_id}.: '{_err}'."),
        }
    }

    pub async fn get_user_dialog_context(chat_id: i64, bot_id: i64) -> Option<String> {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT dialog_context from main WHERE chat_id = ");
        q.push_bind(chat_id);
        q.push(" AND bot_id = ");
        q.push_bind(bot_id);
        q.push(" LIMIT 1");
        let res = q.build().fetch_one(pool);

        match res.await {
            Ok(_ok) => {
                debug!(
                    "📗 Get dialog context for user #{chat_id} is successfully completed: {:?}.",
                    _ok
                );
                _ok.get(0)
            }
            Err(_err) => {
                debug!("📙 Empty result of query get_user_dialog_context: '{_err}'.");
                None
            }
        }
    }

    pub async fn set_user_dialog_context(chat_id: i64, bot_id: i64, context: &str) {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE main SET dialog_context = ");
        q.push_bind(context);
        q.push(", updated_at = current_timestamp WHERE chat_id = ");
        q.push_bind(chat_id);
        q.push(" AND bot_id = ");
        q.push_bind(bot_id);
        let res = q.build().execute(pool).await;

        let now_str = get_now_str();
        match res {
            Ok(_ok) => debug!("📗 Set dialog context \"{context}\" for user #{chat_id}."),
            Err(_err) => {
                println!(
                    "[{now_str}] 📕 - Error on updating dialog context for user #{chat_id} to \"{context}\".: '{_err}'."
                );
                error!(
                    "📕 Error on updating dialog context for user #{chat_id} to \"{context}\".: '{_err}'."
                );
            }
        }
    }

    pub async fn get_user_app_lang(chat_id: i64, bot_id: i64) -> Option<String> {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT app_lang from main WHERE chat_id = ");
        q.push_bind(chat_id);
        q.push(" AND bot_id = ");
        q.push_bind(bot_id);
        q.push(" LIMIT 1");
        let res = q.build().fetch_one(pool);

        match res.await {
            Ok(_ok) => {
                debug!(
                    "📗 Get app_lang for user #{chat_id} is successfully completed: {:?}.",
                    &_ok
                );
                _ok.get(0)
            }
            Err(_err) => {
                debug!("📙 Empty result of query app_lang: '{_err}'.");
                None
            }
        }
    }

    pub async fn set_user_app_lang(chat_id: i64, bot_id: i64, app_lang: &str) {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE main SET app_lang = ");
        q.push_bind(app_lang);
        q.push(", updated_at = current_timestamp WHERE chat_id = ");
        q.push_bind(chat_id);
        q.push(" AND bot_id = ");
        q.push_bind(bot_id);
        let res = q.build().execute(pool).await;

        let now_str = get_now_str();
        match res {
            Ok(_ok) => debug!("📗 Set app_lang \"{app_lang}\" for user #{chat_id}."),
            Err(_err) => {
                println!(
                    "[{now_str}] 📕 - Error on updating app_lang for user #{chat_id} to \"{app_lang}\".: '{_err}'."
                );
                error!(
                    "📕 Error on updating app_lang for user #{chat_id} to \"{app_lang}\".: '{_err}'."
                );
            }
        }
    }

    pub async fn get_user_data(chat_id: i64, bot_id: i64) -> Option<user_data> {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT user_data as Json from main WHERE chat_id = ");
        q.push_bind(chat_id);
        q.push(" AND bot_id = ");
        q.push_bind(bot_id);
        q.push(" LIMIT 1");
        let res = q.build().fetch_one(pool).await;

        let now_str = get_now_str();
        match res {
            Ok(_ok) => {
                let user_data_json = &_ok.try_get_raw(0).unwrap().as_str().unwrap()[1..];
                let parse_to_user_data = serde_json::from_str::<user_data>(user_data_json);
                match parse_to_user_data {
                    Ok(_ok) => {
                        debug!("📗 Get User_data for user #{chat_id} is successfully completed.");
                        Option::from(_ok)
                    }
                    Err(_err) => {
                        println!("[{now_str}] 📕 - Get User_data & parse to Rules for user #{chat_id} has error: {}", _err);
                        error!(
                            "📕 Get User_data & parse to User_data for user #{chat_id} has error: {}",
                            _err
                        );
                        None
                    }
                }
            }
            Err(_err) => {
                error!("📙 Empty result of query get_pgen_rules: '{_err}'.");
                None
            }
        }
    }

    pub async fn get_data_for_statistics(
        col_names: String,
        index_params: IndexParams,
    ) -> Option<Vec<PgRow>> {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> = QueryBuilder::new("SELECT ");
        q.push(col_names);
        q.push(" FROM main");

        if index_params.filter_field.clone().unwrap() != "".to_string()
            && index_params.filter_value.clone().unwrap() != "".to_string()
        {
            q.push(" WHERE ");
            q.push(index_params.filter_field.clone().unwrap());
            q.push(" = \'");
            q.push(index_params.filter_value.clone().unwrap());
            q.push("\'");
        }

        q.push(" ORDER BY ");
        q.push(index_params.sort.unwrap_or("created_at".to_string()));

        if index_params.desc.clone().unwrap() == "off".to_string() {
            q.push(" DESC");
        }

        if index_params.rows_count.clone().unwrap() != "".to_string() {
            q.push(" LIMIT ");
            q.push(index_params.rows_count.clone().unwrap());
        }

        let res = q.build().fetch_all(pool);

        match res.await {
            Ok(_ok) => {
                debug!("📗 Execute get_data_for_statistics is successfully completed.");
                Option::from(_ok)
            }
            Err(_err) => {
                debug!("📙 Bad result of query get_data_for_statistics: '{_err}'.");
                None
            }
        }
    }

    pub async fn web_state_token_existence_check(token: &str) -> bool {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT COUNT(id)::int from web_stat WHERE token = ");
        q.push_bind(token);
        let res = q.build().fetch_one(pool).await;

        match res {
            Ok(_ok) => {
                let token_count: i32 = _ok.get(0);
                debug!(
                    "📗 Check token existence is successfully completed: {:?}.",
                    &token_count
                );
                if token_count > 0 {
                    true
                } else {
                    false
                }
            }
            Err(_err) => {
                debug!("📙 Empty result of query token: '{_err}'.");
                false
            }
        }
    }

    pub async fn get_web_state_token(chat_id: i64, bot_id: i64) -> Option<String> {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT token from web_stat WHERE chat_id = ");
        q.push_bind(chat_id);
        q.push(" AND bot_id = ");
        q.push_bind(bot_id);
        q.push(" LIMIT 1");
        let res = q.build().fetch_one(pool);

        match res.await {
            Ok(_ok) => {
                debug!(
                    "📗 Get token for user #{chat_id} on bot #{bot_id} is successfully completed: {:?}.",
                    &_ok
                );
                _ok.get(0)
            }
            Err(_err) => {
                debug!("📙 Empty result of query token: '{_err}'.");
                None
            }
        }
    }

    pub async fn set_web_state_token(chat_id: i64, bot_id: i64, token: &str) {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> =
            QueryBuilder::new("INSERT INTO web_stat (chat_id, bot_id, token) VALUES (");
        q.push_bind(chat_id);
        q.push(",");
        q.push_bind(bot_id);
        q.push(",");
        q.push_bind(token);
        q.push(") ON CONFLICT (chat_id, bot_id) DO UPDATE SET token = ");
        q.push_bind(token);
        q.push(", updated_at = current_timestamp");
        let res = q.build().execute(pool).await;

        let now_str = get_now_str();
        match res {
            Ok(_ok) => debug!("📗 Set web_stat token for user #{chat_id} on bot #{bot_id}"),
            Err(_err) => {
                println!(
                    "[{now_str}] 📕 - Error on updating web_stat token for user #{chat_id} on bot #{bot_id}: '{_err}'."
                );
                error!(
                    "📕 Error on updating web_stat token for user #{chat_id} on bot #{bot_id}: '{_err}'."
                );
            }
        }
    }

    pub async fn get_users_count() -> i32 {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        //let res: Result<i32, sqlx::Error> = sqlx::query_scalar("SELECT COUNT(id)::int from main").fetch_one(pool).await;
        let res = QueryBuilder::new("SELECT COUNT(id)::int from main")
            .build()
            .fetch_one(pool)
            .await;

        match res {
            Ok(_ok) => {
                debug!(
                    "📗 Get the summary users count of bots is successfully completed: {:?}.",
                    &_ok
                );
                _ok.get(0)
            }
            Err(_err) => {
                debug!("📕 Error on getting of the summary users count of bots: '{_err}'.");
                -1
            }
        }
    }

    pub async fn get_total_pwds_sum() -> i32 {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let res = QueryBuilder::new("SELECT SUM(gen_count)::int from main")
            .build()
            .fetch_one(pool)
            .await;

        match res {
            Ok(_ok) => {
                debug!(
                    "📗 Get the total count of generated passwords by all bots is successfully completed: {:?}.",
                    &_ok
                );
                _ok.get(0)
            }
            Err(_err) => {
                debug!("📕 Error on getting of the total count of generated passwords by all bots: '{_err}'.");
                -1
            }
        }
    }
}
