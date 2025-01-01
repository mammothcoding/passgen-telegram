pub mod user {

    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
    pub struct User {
        pub username: String,
        pub first_name: String,
        pub last_name: String,
        pub language_code: String,
        pub is_bot: bool,
    }
}
