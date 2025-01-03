pub mod log {
    use crate::env_processing::env_processing::DotEnv;
    use log::{info, LevelFilter};
    use log4rs::{
        append::{
            console::{ConsoleAppender, Target},
            rolling_file::policy::compound::{
                roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
            },
        },
        config::{Appender, Config, Root},
        encode::pattern::PatternEncoder,
        filter::threshold::ThresholdFilter,
    };
    use crate::get_now_str;

    const FILE_PATH: &str = "log/passgen-tg.log";
    const ARCHIVE_PATTERN: &str = "log/passgen-tg.{}.log.gz";

    pub fn init(env_data: &DotEnv) {
        // Build a stderr logger.
        let stderr_appr = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "[{d(%d-%b-%y %X%.6f %Z)}] {h({l})} - {m}\n",
            )))
            .target(Target::Stderr)
            .build();
        let stderr_lvl = obtain_lvl_filter(&env_data.log_stderr_lvl); //LevelFilter::Info;

        let config = if &env_data.log_logfile_lvl != "off" {
            // Create a policy to use with the file logging
            let trigger = SizeTrigger::new(env_data.log_trigger_file_size);
            let roller = FixedWindowRoller::builder()
                .base(0) // Default Value (line not needed unless you want to change from 0 (only here for demo purposes)
                .build(ARCHIVE_PATTERN, env_data.log_files_count)
                .expect("🚫 Log roller setup error!");
            let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));

            // Logging to log file. (with rolling)
            let logfile_appr = log4rs::append::rolling_file::RollingFileAppender::builder()
                // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
                .encoder(Box::new(PatternEncoder::new(
                    "[{d(%d-%b-%y %X%.6f %Z)}] {l} - {m}\n",
                )))
                .build(FILE_PATH, Box::new(policy))
                .expect("🚫 Log logfile_appr setup error!");
            let logfile_lvl = obtain_lvl_filter(&env_data.log_logfile_lvl); //LevelFilter::Debug;

            Config::builder()
                .appender(
                    Appender::builder()
                        .filter(Box::new(ThresholdFilter::new(stderr_lvl)))
                        .build("stderr_appr", Box::new(stderr_appr)),
                )
                .appender(
                    Appender::builder()
                        .filter(Box::new(ThresholdFilter::new(logfile_lvl)))
                        .build("logfile_appr", Box::new(logfile_appr)),
                )
                .build(
                    Root::builder()
                        .appender("stderr_appr")
                        .appender("logfile_appr")
                        .build(LevelFilter::Trace),
                )
                .expect("🚫 Log config setup error!")
        } else {
            Config::builder()
                .appender(
                    Appender::builder()
                        .filter(Box::new(ThresholdFilter::new(stderr_lvl)))
                        .build("stderr_appr", Box::new(stderr_appr)),
                )
                .build(
                    Root::builder()
                        .appender("stderr_appr")
                        .build(LevelFilter::Trace),
                )
                .expect("🚫 Log config setup error!")
        };

        let _handle = log4rs::init_config(config).expect("🚫 Log init_config error!");

        let now_str = get_now_str();
        println!("[{now_str}] ✅ - log init is OK");
        info!("✅ - log init is OK");
    }

    fn obtain_lvl_filter(lvl: &String) -> LevelFilter {
        match lvl.as_ref() {
            "trace" => LevelFilter::Trace,
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            "off" => LevelFilter::Off,
            _ => LevelFilter::Off,
        }
    }
}
