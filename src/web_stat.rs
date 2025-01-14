pub mod web_stat {
    use crate::get_now_str;
    use axum::body::Body;
    use axum::extract::Path;
    use axum::http::{header, StatusCode};
    use axum::response::IntoResponse;
    use axum::routing::get;
    use axum::Router;
    use passgenlib::Passgen;
    use std::env;
    use std::sync::{Mutex, OnceLock};
    use tokio_util::io::ReaderStream;

    // [0] - auth token
    // [1] - total bots users count
    // [2] - total bots generated passwords
    pub fn web_state() -> &'static Mutex<[&'static str; 3]> {
        static ARRAY: OnceLock<Mutex<[&'static str; 3]>> = OnceLock::new();
        ARRAY.get_or_init(|| Mutex::new(["", "", ""]))
    }

    pub fn gen_token() {
        let token = Passgen::new()
            .set_enabled_letters(true)
            .set_enabled_numbers(true)
            .generate(30);
        web_state().lock().unwrap()[0] = token.leak();
    }

    pub fn get_web_state_token() -> &'static str {
        web_state().lock().unwrap()[0]
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

    async fn web_stat_handler(Path(token): Path<String>) -> Body {
        let body_string = if token == get_web_state_token() {
            "<body style=\"background-color:black; text-align:center; color:white\">
<h3>stat</h3>
</body>".to_string()
        } else {
"<body style=\"background-color:black; text-align:center; color:white\">
<h3>&#x26A0; Incorrect token!</h3>
<h3>Please reload statistics menu and follow throw new link</h3>
</body>".to_string()
        };

        let body = Body::from(body_string);
        body
    }

    pub fn get_router() -> Router {
        Router::new()
            .route("/{token}", get(web_stat_handler))
            .route("/favicon.ico", get(get_favicon))
    }
}
