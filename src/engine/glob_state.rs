pub mod glob_state {
    use std::sync::{Mutex, OnceLock};

    // [0] - current bot_name
    // [1] - the summary users count of bots
    // [2] - the total count of generated passwords by all bots
    pub fn state() -> &'static Mutex<[&'static str; 3]> {
        //static VEC: OnceLock<Mutex<Vec(&'static str)>> = OnceLock::new();
        static ARRAY: OnceLock<Mutex<[&'static str; 3]>> = OnceLock::new();
        ARRAY.get_or_init(|| Mutex::new(["", "-1", "-1"]))
    }

    pub fn set_bot_name(bot_name: &String) {
        state().lock().unwrap()[0] = bot_name.clone().leak();
    }

    pub fn get_bot_name() -> String {
        state().lock().unwrap()[0].to_string()
    }

    pub fn set_bots_users_count(users_count: i32) {
        state().lock().unwrap()[1] = users_count.to_string().clone().leak();
    }

    pub fn get_bots_users_count() -> String {
        state().lock().unwrap()[1].to_string()
    }

    pub fn set_bots_gen_pwds(passwords_count: i32) {
        state().lock().unwrap()[2] = passwords_count.to_string().clone().leak();
    }

    pub fn get_bots_gen_pwds() -> String {
        state().lock().unwrap()[2].to_string()
    }
}
