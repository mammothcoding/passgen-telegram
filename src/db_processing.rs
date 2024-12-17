pub mod db_processing {
    use crate::env_processing::env_processing::DotEnv;
    use sqlx::{Connection, Executor, PgConnection, PgPool, Pool, Postgres, QueryBuilder, Row};
    use std::process;
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
            let now_str = chrono::Local::now().format("%d-%b-%y %X%.6f").to_string();
            println!("🚫 [{now_str}] Error on connection to Postgres: '{}'.", _err);
            process::exit(1);
        }

        let check_or_create_db_res = connection
            .unwrap()
            .execute(format!(r#"CREATE DATABASE "{}";"#, env_data.db_name).as_str())
            .await;
        if let Err(_err) = &check_or_create_db_res {
            let now_str = chrono::Local::now().format("%d-%b-%y %X%.6f").to_string();
            if _err.to_string().contains("already exists") {
                println!("✅ [{now_str}] DB '{}' is exists.", env_data.db_name);
            } else {
                println!("🚫 [{now_str}] Error check_or_create_db: '{}'.", _err);
                process::exit(1);
            }
        }
        if check_or_create_db_res.is_ok() {
            let now_str = chrono::Local::now().format("%d-%b-%y %X%.6f").to_string();
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
            let now_str = chrono::Local::now().format("%d-%b-%y %X%.6f").to_string();
            println!("🚫 [{now_str}] Error on PgPool::connect: '{}'", _err);
            process::exit(1);
        }
        let now_str = chrono::Local::now().format("%d-%b-%y %X%.6f").to_string();
        println!("✅ [{now_str}] Connect to '{}' DB.", env_data.db_name);

        let pool = connection_pool.unwrap();
        let migr_res = sqlx::migrate!("./migrations").run(&pool).await;
        let now_str = chrono::Local::now().format("%d-%b-%y %X%.6f").to_string();
        if let Err(_err) = migr_res {
            println!("🚫 [{now_str}] Error on migrates: '{}'.", _err);
            process::exit(1);
        }
        println!("✅ [{now_str}] DB migrating status.");

        DB_POOL.get_or_init(|| async { pool }).await;
    }

    pub async fn get_last_user_mess_id(chat_id: i64) -> Option<i32> {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> =
            QueryBuilder::new("SELECT message from main WHERE id = ");
        q.push_bind(chat_id);
        q.push(" LIMIT 1");
        let res = q.build().fetch_one(pool);

        let now_str = chrono::Local::now().format("%d-%b-%y %X%.6f").to_string();
        match res.await {
            Ok(_ok) => {
                println!("📗 [{now_str}] Query get_last_user_mess_id successfully completed: {:?}.", _ok);
                _ok.get(0)
            },
            Err(_err) => {
                println ! ("📙 [{now_str}] Empty result of query get_last_user_mess_id: '{_err}'.");
                None
            },
        }
    }

    pub async fn user_press_inline_btn(chat_id: i64, mess_id: i32) {
        let pool = DB_POOL.get().expect("DB_POOL.get().expect");
        let mut q: QueryBuilder<Postgres> =
            QueryBuilder::new("INSERT INTO main (id, message) VALUES (");
        q.push_bind(chat_id);
        q.push(",");
        q.push_bind(mess_id);
        q.push(") ON CONFLICT (id) DO UPDATE SET message = ");
        q.push_bind(mess_id);
        let res = q.build().execute(pool).await;

        let now_str = chrono::Local::now().format("%d-%b-%y %X%.6f").to_string();
        match res {
            Ok(_ok) => println!("📗 [{now_str}] Register new mess #{mess_id} of gen password successfully completed: {:?}.", _ok),
            Err(_err) => println!("📕 [{now_str}] Error on query of register new mess #{mess_id} of gen password: '{_err}'."),
        }
    }
}
