pub mod log {
    use log::{debug, error, info, trace, warn, LevelFilter};
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
    use log4rs::encode::{Encode};
    use crate::env_processing::env_processing::DotEnv;

    const TRIGGER_FILE_SIZE: u64 = 512 * 1024;

    const LOG_FILE_COUNT: u32 = 5;

    const FILE_PATH: &str = "log/passgen-tg.log";

    /// Location where log archives will be moved to
    /// For Pattern info See:
    /// https://docs.rs/log4rs/*/log4rs/append/rolling_file/policy/compound/roll/fixed_window/struct.FixedWindowRollerBuilder.html#method.build
    const ARCHIVE_PATTERN: &str = "log/passgen-tg.{}.log.gz";

    pub fn init(env_data: &DotEnv) {
        // Build a stderr logger.
        let stderr_appr = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new("[{d(%d-%b-%y %X%.6f %Z)}] {h({l})} - {m}\n")))
            .target(Target::Stderr)
            .build();
        let stderr_lvl = LevelFilter::Info;

        // Create a policy to use with the file logging
        let trigger = SizeTrigger::new(TRIGGER_FILE_SIZE);
        let roller = FixedWindowRoller::builder()
            .base(0) // Default Value (line not needed unless you want to change from 0 (only here for demo purposes)
            .build(ARCHIVE_PATTERN, LOG_FILE_COUNT)
            .expect("🚫 Log roller setup error!");
        let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));

        // Logging to log file. (with rolling)
        let logfile_appr = log4rs::append::rolling_file::RollingFileAppender::builder()
            // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
            .encoder(Box::new(PatternEncoder::new("[{d(%d-%b-%y %X%.6f %Z)}] {l} - {m}\n")))
            .build(FILE_PATH, Box::new(policy))
            .expect("🚫 Log logfile_appr setup error!");
        let logfile_lvl = LevelFilter::Debug;

        let config = Config::builder()
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(stderr_lvl)))
                    .build("stderr_appr", Box::new(stderr_appr))
            )
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(logfile_lvl)))
                    .build("logfile_appr", Box::new(logfile_appr)))
            .build(
                Root::builder()
                    .appender("stderr_appr")
                    .appender("logfile_appr")
                    .build(LevelFilter::Trace),
            )
            .expect("🚫 Log config setup error!");

        let _handle = log4rs::init_config(config).expect("🚫 Log init_config error!");
    }
}
