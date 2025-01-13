pub mod web_stat {
    use passgenlib::Passgen;
    use std::sync::{Mutex, OnceLock};

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
}
