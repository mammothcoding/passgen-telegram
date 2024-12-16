pub mod db_processing {
    use crate::env_processing::env_processing::DotEnv;
    use sqlx::{Connection, Executor, PgConnection, PgPool, Pool, Postgres};
    use std::process;

    pub async fn init(env_data: &DotEnv) -> Pool<Postgres> {
        let connection = PgConnection::connect(
            format!(
                "postgres://{}:{}@{}:{}",
                env_data.db_username, env_data.db_password, env_data.db_host, env_data.db_port
            )
            .as_str(),
        )
        .await;
        if let Err(_err) = &connection {
            println!("🚫 Error on connection to Postgres: '{}'", _err);
            process::exit(1);
        }

        let check_or_create_db_res = connection
            .unwrap()
            .execute(format!(r#"CREATE DATABASE "{}";"#, env_data.db_name).as_str())
            .await;
        if let Err(_err) = &check_or_create_db_res {
            if _err.to_string().contains("already exists") {
                println!("✅ DB '{}' is exists.", env_data.db_name);
            } else {
                println!("🚫 Error check_or_create_db: '{}'", _err);
                process::exit(1);
            }
        }
        if check_or_create_db_res.is_ok() {
            println!(
                "✅ DB '{}' doesn't exist. Create new DB successfully.",
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
            println!("🚫 Error on PgPool::connect: '{}'", _err);
            process::exit(1);
        }
        println!("✅ Connect to '{}' DB.", env_data.db_name);

        let pool = connection_pool.unwrap();
        let migr_res = sqlx::migrate!("./migrations").run(&pool).await;
        if let Err(_err) = migr_res {
            println!("🚫 Error on migrates: '{}'", _err);
            process::exit(1);
        }
        println!("✅ DB migrating status.");

        pool
    }

    pub async fn user_press_inline_btn(pool: &Pool<Postgres>, chat_id: u64, mess_id: i32) {
        let q = sqlx::query(
            "INSERT INTO main (id, message)
            VALUES ($1, $2)
            ON CONFLICT (id) DO UPDATE SET
            message = $2",
        )
        .bind(chat_id)
        .bind(mess_id.to_string())
        .execute(pool)
        .await;

        match q {
            Ok(_ok) => println!("📗 Query successfully completed: {:?}", _ok),
            Err(_err) => println!("📕 Error on query: '{_err}'"),
        }
    }
}
