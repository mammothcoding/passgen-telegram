pub mod db_processing {
    use crate::env_processing::env_processing::DotEnv;
    use crate::{get_now_str, Rules};
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
                "🚫 [{now_str}] Error on connection to Postgres: '{}'.",
                _err
            );
            process::exit(1);
        }

        let check_or_create_db_res = connection
            .unwrap()
            .execute(format!(r#"CREATE DATABASE "{}";"#, env_data.db_name).as_str())
            .await;
        if let Err(_err) = &check_or_create_db_res {
            let now_str = get_now_str();
            if _err.to_string().contains("already exists") {
                println!("✅ [{now_str}] DB '{}' is exists.", env_data.db_name);
            } else {
                println!("🚫 [{now_str}] Error check_or_create_db: '{}'.", _err);
                process::exit(1);
            }
        }
        if check_or_create_db_res.is_ok() {
            let now_str = get_now_str();
            println!(
                "✅ [{now_str}] DB '{}' doesn't exist. Create new DB successfully.",
                env_data.db_name
            );
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
            println!("🚫 [{now_str}] Error on PgPool::connect: '{}'", _err);
            process::exit(1);
        }
        let now_str = get_now_str();
        println!("✅ [{now_str}] Connect to '{}' DB.", env_data.db_name);

        let pool = connection_pool.unwrap();
        let migr_res = sqlx::migrate!("./migrations").run(&pool).await;
        let now_str = get_now_str();
        if let Err(_err) = migr_res {
            println!("🚫 [{now_str}] Error on migrates: '{}'.", _err);
            process::exit(1);
        }
        println!("✅ [{now_str}] DB migrating status is OK.");

        DB_POOL.get_or_init(|| async { pool }).await;
    }

    pub async fn check_user_rec_avail(chat_id: i64) -> bool {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> = QueryBuilder::new("SELECT id from main WHERE id = ");
        q.push_bind(chat_id);
        q.push(" LIMIT 1");
        let res = q.build().fetch_one(pool);

        let now_str = get_now_str();
        match res.await {
            Ok(_ok) => {
                println!(
                    "📗 [{now_str}] Query check_user_rec_avail successfully completed: {:?}.",
                    _ok
                );
                true
            }
            Err(_err) => {
                println!("📙 [{now_str}] Empty result of query check_user_rec_avail: '{_err}'.");
                false
            }
        }
    }

    pub async fn cr_new_user_rec(chat_id: i64, from: User) -> bool {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> = QueryBuilder::new(
            "INSERT INTO main (
            id, is_bot, first_name, last_name, username, language_code) VALUES (",
        );
        q.push_bind(chat_id);
        q.push(",");
        q.push_bind(from.is_bot);
        q.push(",");
        q.push_bind(&from.first_name);
        q.push(",");
        q.push_bind(&from.last_name);
        q.push(",");
        q.push_bind(&from.username);
        q.push(",");
        q.push_bind(&from.language_code);
        q.push(")");
        let res = q.build().execute(pool).await;

        let now_str = get_now_str();
        match res {
            Ok(_ok) => {
                println!(
                    "👤 [{now_str}] New user reg #{chat_id} [{} {:?} {:?}  {:?}].",
                    from.first_name, from.last_name, from.username, from.language_code
                );
                true
            }
            Err(_err) => {
                println!("📕 [{now_str}] Error on reg new user #{chat_id} in DB: '{_err}'.");
                false
            }
        }
    }

    pub async fn get_pgen_rules(chat_id: i64) -> Option<Rules> {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT pgen_rules as Json from main WHERE id = ");
        q.push_bind(chat_id);
        q.push(" LIMIT 1");
        let res = q.build().fetch_one(pool).await;

        let now_str = get_now_str();
        match res {
            Ok(_ok) => {
                let pgen_rules_json = &_ok.try_get_raw(0).unwrap().as_str().unwrap()[1..];
                let parse_to_rules = serde_json::from_str::<Rules>(pgen_rules_json);
                match parse_to_rules {
                    Ok(_ok) => {
                        println!("📗 [{now_str}] Get Rules for user #{chat_id} is successfully.");
                        Option::from(_ok)
                    }
                    Err(_err) => {
                        println!("📕 Get Rules for user #{chat_id} has error: {}", _err);
                        None
                    }
                }
            }
            Err(_err) => {
                println!("📙 [{now_str}] Empty result of query get_pgen_rules: '{_err}'.");
                None
            }
        }
    }

    pub async fn get_last_mess_id(chat_id: i64, id_field: &str) -> Option<i32> {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> =
            QueryBuilder::new(format!("SELECT {id_field} from main WHERE id = "));
        q.push_bind(chat_id);
        q.push(" LIMIT 1");
        let res = q.build().fetch_one(pool);

        let now_str = get_now_str();
        match res.await {
            Ok(_ok) => {
                println!(
                    "📗 [{now_str}] Query get_last_user_mess_id successfully completed: {:?}.",
                    _ok
                );
                _ok.get(0)
            }
            Err(_err) => {
                println!("📙 [{now_str}] Empty result of query get_last_user_mess_id: '{_err}'.");
                None
            }
        }
    }

    pub async fn set_last_mess_id(chat_id: i64, mess_id: i32, id_field: &str) {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> =
            QueryBuilder::new(format!("UPDATE main SET {id_field} = "));
        q.push_bind(mess_id);
        q.push(", updated_at = current_timestamp");
        q.push(" WHERE id = ");
        q.push_bind(chat_id);
        let res = q.build().execute(pool).await;

        let now_str = get_now_str();
        match res {
            Ok(_ok) => println!("📗 [{now_str}] Register new mess id #{mess_id} to {id_field} field."),
            Err(_err) => println!("📕 [{now_str}] Error on query of register new mess id #{mess_id} to {id_field} field: '{_err}'."),
        }
    }

    pub async fn update_rules(chat_id: i64, rules: Rules) -> bool {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE main SET pgen_rules = ");
        q.push_bind(Json(rules));
        q.push(" WHERE id = ");
        q.push_bind(chat_id);
        let res = q.build().execute(pool).await;

        let now_str = get_now_str();
        match res {
            Ok(_ok) => {
                println!("📗 [{now_str}] Rules has been updated for user #{chat_id}.");
                true
            }
            Err(_err) => {
                println!("📕 [{now_str}] Error on updating rules for user #{chat_id}.: '{_err}'.");
                false
            }
        }
    }

    pub async fn increase_user_gen_count(chat_id: i64) {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> = QueryBuilder::new(
            "UPDATE main SET gen_count = gen_count + 1, updated_at = current_timestamp",
        );
        q.push(" WHERE id = ");
        q.push_bind(chat_id);
        let res = q.build().execute(pool).await;

        let now_str = get_now_str();
        match res {
            Ok(_ok) => println!("📗 [{now_str}] Increase gen_count for user #{chat_id}."),
            Err(_err) => println!(
                "📕 [{now_str}] Error on increase gen_count for user #{chat_id}.: '{_err}'."
            ),
        }
    }

    pub async fn get_user_dialog_context(chat_id: i64) -> Option<String> {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT dialog_context from main WHERE id = ");
        q.push_bind(chat_id);
        q.push(" LIMIT 1");
        let res = q.build().fetch_one(pool);

        let now_str = get_now_str();
        match res.await {
            Ok(_ok) => {
                println!(
                    "📗 [{now_str}] Get dialog context for user #{chat_id} successfully completed: {:?}.",
                    _ok
                );
                _ok.get(0)
            }
            Err(_err) => {
                println!("📙 [{now_str}] Empty result of query get_user_dialog_context: '{_err}'.");
                None
            }
        }
    }

    pub async fn set_user_dialog_context(chat_id: i64, context: &str) {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE main SET dialog_context = ");
        q.push_bind(context);
        q.push(", updated_at = current_timestamp WHERE id = ");
        q.push_bind(chat_id);
        let res = q.build().execute(pool).await;

        let now_str = get_now_str();
        match res {
            Ok(_ok) => println!("📗 [{now_str}] Set dialog context \"{context}\" for user #{chat_id}."),
            Err(_err) => println!(
                "📕 [{now_str}] Error on updating dialog context for user #{chat_id} to \"{context}\".: '{_err}'."
            ),
        }
    }
}
