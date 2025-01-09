pub mod web_stat {
    use std::sync::{Mutex, OnceLock};

    fn array() -> &'static Mutex<[&'static str; 3]> {
        static ARRAY: OnceLock<Mutex<[&'static str; 3]>> = OnceLock::new();
        ARRAY.get_or_init(|| Mutex::new(["", "", ""]))
    }

    fn do_a_call() {
        array().lock().unwrap().push(1);
    }
}