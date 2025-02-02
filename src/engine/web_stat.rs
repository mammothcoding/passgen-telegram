pub mod web_stat {
    use crate::engine::db_processing::db_processing::{
        get_data_for_statistics, set_web_state_token, web_state_token_existence_check,
    };
    use crate::engine::env_processing::env_processing::DotEnv;
    use crate::engine::glob_state::glob_state::{
        get_bots_gen_pwds as glob_st_get_bots_gen_pwds,
        get_bots_users_count as glob_st_get_bots_users_count,
    };
    use crate::get_now_str;
    use crate::structs::user::user::User;
    use axum::body::Body;
    use axum::extract::{Path, Query, State};
    use axum::http::{header, StatusCode};
    use axum::response::IntoResponse;
    use axum::routing::get;
    use axum::Router;
    use chrono::DateTime;
    use passgenlib::Passgen;
    use serde::{Deserialize, Serialize};
    use sqlx::types::Json;
    use sqlx::Row;
    use std::env;
    use tokio_util::io::ReaderStream;

    pub async fn create_token(chat_id: i64, bot_id: i64) {
        let token = Passgen::new()
            .set_enabled_letters(true)
            .set_enabled_numbers(true)
            .generate(30);
        set_web_state_token(chat_id, bot_id, &token.leak()).await
    }

    pub struct WebStatTableCols {
        created_at: DateTime<chrono::Local>,
        updated_at: DateTime<chrono::Local>,
        chat_id: i64,
        user_data: Json<User>,
        app_lang: String,
        gen_count: i64,
        bot_id: i64,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct IndexParams {
        pub sort: Option<String>,
        pub desc: Option<String>,
        pub filter_field: Option<String>,
        pub filter_value: Option<String>,
        pub rows_count: Option<String>,
    }

    #[derive(Serialize)]
    #[allow(non_snake_case)]
    struct GithubBadgeJson {
        schemaVersion: u8,
        label: String,
        message: String,
        color: String,
    }

    impl IndexParams {
        fn fill_according_the_request(&self) -> Self {
            Self {
                sort: Option::from(self.clone().sort.unwrap_or("created_at".to_string())),
                desc: Option::from(self.clone().desc.unwrap_or("off".to_string())),
                filter_field: Option::from(self.clone().filter_field.unwrap_or("".to_string())),
                filter_value: Option::from(self.clone().filter_value.unwrap_or("".to_string())),
                rows_count: Option::from(self.clone().rows_count.unwrap_or("100".to_string())),
            }
        }
    }

    pub async fn get_bots_users_count() -> impl IntoResponse {
        let users_count = glob_st_get_bots_users_count();
        let badge_data = GithubBadgeJson {
            schemaVersion: 1,
            label: "tg_users".to_string(),
            message: users_count.to_string(),
            color: "lime".to_string(),
        };
        let body = Json(badge_data).encode_to_string().unwrap();
        let headers = [(header::CONTENT_TYPE, "application/json")];

        (headers, body)
    }

    pub async fn get_bots_gen_pwds() -> impl IntoResponse {
        let passwords_count = glob_st_get_bots_gen_pwds();
        let badge_data = GithubBadgeJson {
            schemaVersion: 1,
            label: "gens_count".to_string(),
            message: passwords_count.to_string(),
            color: "coral".to_string(),
        };
        let body = Json(badge_data).encode_to_string().unwrap();
        let headers = [(header::CONTENT_TYPE, "application/json")];

        (headers, body)
    }

    pub async fn get_favicon() -> impl IntoResponse {
        let now_str = get_now_str();
        let cur_exe_binding =
            env::current_exe().expect(&format!("[{now_str}] 🚫 - Get cur_exe_binding path error!"));
        let path = &cur_exe_binding.with_file_name("favicon.ico");

        let file = match tokio::fs::File::open(path).await {
            Ok(file) => file,
            Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
        };
        let content_type = match mime_guess::from_path(&path).first_raw() {
            Some(mime) => mime,
            None => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "MIME Type couldn't be determined".to_string(),
                ))
            }
        };
        let stream = ReaderStream::new(file);
        let body = Body::from_stream(stream);
        let headers = [
            (header::CONTENT_TYPE, content_type),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"favicon.ico\"",
            ),
        ];
        Ok((headers, body))
    }

    async fn web_stat_handler(
        Path(token): Path<String>,
        q_index_params: Query<IndexParams>,
        State(env): State<DotEnv>,
    ) -> Body {
        let mut body_stack = Vec::new();

        if web_state_token_existence_check(&token[..]).await {
            let col_names = [
                "created_at",
                "updated_at",
                "chat_id",
                "user_data",
                "app_lang",
                "gen_count",
                "bot_id",
            ];
            let index_params: IndexParams = q_index_params.0.fill_according_the_request();

            body_stack.push("<!doctype html><html>".to_string());
            body_stack.push(
                "<head>
<meta http-equiv=\"Content-Type\" content=\"text/html; charset=utf-8\">
<title>bots-stats</title>
<style>
    title {
        text-align: center;
    }
    body {
        background-color: rgb(33, 44, 55);
        color: white;
        text-align: center;
    }
    table {
        border: 3px double;
        border-collapse: collapse;
        width: auto;
        margin: auto;
    }
    td, th {
        padding: 10px;
        border: 1px solid;
        text-align: center;
        vertical-align: center;
        width: auto;
    }
</style>
        </head>"
                    .to_string(),
            );
            body_stack.push("<body>".to_string());
            body_stack.push("<h2>Bots statistics:</h2>".to_string());

            let users_count = glob_st_get_bots_users_count();
            let passwords_count = glob_st_get_bots_gen_pwds();
            body_stack.push(
                [
                    "<h3>total &#x1F464;: ".to_string(),
                    users_count,
                    ",  total &#x1F3B2;: ".to_string(),
                    passwords_count,
                    "</h3>".to_string(),
                ]
                .join(""),
            );

            let db_data =
                get_data_for_statistics(col_names.clone().join(","), index_params.clone()).await;
            match db_data {
                Some(data) => {
                    body_stack.push("<form method=\"get\">".to_string());

                    body_stack.push("&#x1F503;".to_string());
                    body_stack.push("<select name=\"sort\" autofocus>".to_string());
                    body_stack.push(
                        col_names
                            .iter()
                            .map(|col_name| {
                                if col_name.to_string() == index_params.sort.clone().unwrap() {
                                    ["<option selected>", &col_name, "</option>"].join("")
                                } else {
                                    ["<option>", &col_name, "</option>"].join("")
                                }
                            })
                            .collect(),
                    );
                    body_stack.push("</select>".to_string());

                    if index_params.desc.clone().unwrap() == "on".to_string() {
                        body_stack.push(
                            "<input type=\"checkbox\" id=\"desc\" name=\"desc\" checked />"
                                .to_string(),
                        );
                    } else {
                        body_stack.push(
                            "<input type=\"checkbox\" id=\"desc\" name=\"desc\" />".to_string(),
                        );
                    }
                    body_stack.push("<label for=\"desc\">&#x23EB;</label>".to_string());

                    body_stack.push("<span style=\"margin: 10px\"></span>".to_string());

                    body_stack.push("&#x1F50E;".to_string());
                    body_stack.push("<select name=\"filter_field\">".to_string());
                    body_stack.push(
                        ["chat_id", "app_lang", "bot_id"]
                            .iter()
                            .map(|col_name| {
                                if col_name.to_string()
                                    == index_params.filter_field.clone().unwrap()
                                {
                                    ["<option selected>", &col_name, "</option>"].join("")
                                } else {
                                    ["<option>", &col_name, "</option>"].join("")
                                }
                            })
                            .collect(),
                    );
                    body_stack.push("</select>".to_string());
                    body_stack.push(["<input type=\"search\" id=\"filter_value\" name=\"filter_value\" maxlength=\"30\" size=\"10\" value=\"", &*index_params.filter_value.clone().unwrap(), "\"/>"].join(""));

                    body_stack.push("<span style=\"margin: 10px\"></span>".to_string());

                    body_stack.push("<label for=\"rows_count\">rows:</label>".to_string());
                    body_stack.push(["<input type=\"number\" id=\"rows_count\" name=\"rows_count\" size=\"5\" value=\"", &*index_params.rows_count.clone().unwrap(), "\"/>"].join(""));

                    body_stack.push("<span style=\"margin: 10px\"></span>".to_string());

                    body_stack.push(
                        "<input type=\"submit\" name=\"submit\" value=\"&#x2705;OK\">".to_string(),
                    );
                    body_stack.push("</form>".to_string());

                    body_stack.push("<br>".to_string());

                    body_stack.push("<table>".to_string());
                    body_stack.push("<tr>".to_string());
                    body_stack.push(
                        col_names
                            .iter()
                            .map(|col_name| ["<th>", &col_name, "</th>"].join(""))
                            .collect(),
                    );
                    body_stack.push("</tr>".to_string());

                    for row in data.iter() {
                        let row_struct = WebStatTableCols {
                            created_at: row.get(0),
                            updated_at: row.get(1),
                            chat_id: row.get(2),
                            user_data: row.get(3),
                            app_lang: row.get(4),
                            gen_count: row.get(5),
                            bot_id: row.get(6),
                        };

                        body_stack.push(
                            [
                                "<tr>".to_string(),
                                [
                                    "<td>".to_string(),
                                    row_struct.created_at.format("%y-%b-%d %H:%M").to_string(),
                                    "</td>".to_string(),
                                ]
                                .join(""),
                                [
                                    "<td>".to_string(),
                                    row_struct.updated_at.format("%y-%b-%d %H:%M").to_string(),
                                    "</td>".to_string(),
                                ]
                                .join(""),
                                [
                                    "<td>".to_string(),
                                    row_struct.chat_id.to_string(),
                                    "</td>".to_string(),
                                ]
                                .join(""),
                                [
                                    "<td>".to_string(),
                                    row_struct.user_data.first_name.to_string(),
                                    " | ".to_string(),
                                    row_struct.user_data.last_name.to_string(),
                                    " | ".to_string(),
                                    row_struct.user_data.username.to_string(),
                                    "</td>".to_string(),
                                ]
                                .join(""),
                                ["<td>".to_string(), row_struct.app_lang, "</td>".to_string()]
                                    .join(""),
                                [
                                    "<td>".to_string(),
                                    row_struct.gen_count.to_string(),
                                    "</td>".to_string(),
                                ]
                                .join(""),
                                [
                                    "<td>".to_string(),
                                    {
                                        let key = row_struct.bot_id.clone().to_string();
                                        match env.web_stat_bots_usernames.get(&key) {
                                            Some(val) => val.clone(),
                                            _ => key,
                                        }
                                    },
                                    "</td>".to_string(),
                                ]
                                .join(""),
                                "</tr>".to_string(),
                            ]
                            .join(""),
                        );
                    }
                    body_stack.push("</table>".to_string());
                }
                None => {
                    body_stack.push("<h3>Incorrect input data!</h3>".to_string());
                }
            }

            body_stack.push("</body>".to_string());
            body_stack.push("</html>".to_string());
        } else {
            body_stack.push(
                "<body style=\"background-color:black; text-align:center; color:white\">
<h3>&#x26A0; Incorrect token!</h3>
<h3>Please reload statistics menu and follow throw new link</h3>
</body>"
                    .to_string(),
            );
        };

        Body::from(body_stack.join(""))
    }

    pub fn get_router(env: &DotEnv) -> Router {
        Router::new()
            .route("/favicon.ico", get(get_favicon))
            .route("/bots-users-count", get(get_bots_users_count))
            .route("/total-count-generated-pwds", get(get_bots_gen_pwds))
            .route("/{token}", get(web_stat_handler))
            .with_state(env.clone())
    } //
}
