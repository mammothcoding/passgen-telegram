pub mod glob_state {
    use std::sync::{Mutex, OnceLock};

    // [0] - current bot_name
    // [1] - the summary count of bots users
    // [2] - total count of generated passwords by all bots
    pub fn state() -> &'static Mutex<[&'static str; 3]> {
        //static VEC: OnceLock<Mutex<Vec(&'static str)>> = OnceLock::new();
        static ARRAY: OnceLock<Mutex<[&'static str; 3]>> = OnceLock::new();
        ARRAY.get_or_init(|| Mutex::new(["", "", ""]))
    }

    pub fn set_bot_name(bot_name: &String) {
        state().lock().unwrap()[0] = bot_name.clone().leak();
    }

    pub fn get_bot_name() -> String {
        state().lock().unwrap()[0].to_string()
    }
}