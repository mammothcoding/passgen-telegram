pub mod web_stat {
    use std::sync::{Mutex, OnceLock};
    use passgenlib::Passgen;

    // [0] - auth token
    // [1] - total bots users count
    // [2] - total bots generated passwords
    pub fn web_state() -> &'static Mutex<[&'static str; 3]> {
        static ARRAY: OnceLock<Mutex<[&'static str; 3]>> = OnceLock::new();
        ARRAY.get_or_init(|| Mutex::new(["", "", ""]))
    }

    pub fn gen_token() {
        let token = Passgen::new().set_enabled_letters(true).set_enabled_numbers(true).generate(30);
        web_state().lock().unwrap()[0] = token.leak();
    }

    pub fn get_web_state_token() -> &'static str {
        web_state().lock().unwrap()[0]
    }

    /*pub fn get_in_menu_wstat_link_line() -> &'static str {
        gen_token();
        get_web_state_token();
        let line = format!("📊 <b><a href=\"https://github.com/mammothcoding/passgen-telegram\">stat-glob</a></b>");
        line
    }*/
}
