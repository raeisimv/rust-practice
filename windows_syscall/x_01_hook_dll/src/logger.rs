use log4rs::append::file::FileAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::LevelFilter;

pub fn init_logger() {
    let log_file_name = "my-log-file-name.log";
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d.%H:%M:%S)} [{f}:{L}] {l} - {m}\n",
        )))
        .build(log_file_name)
        .expect("[Business Rule]: unable to build logFile");

    let config = Config::builder()
        .appender(
            Appender::builder()
                .build("logfile", Box::new(logfile))
        )
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Info),
        )
        .expect("[Business Rule]: unable to build the log's config");

    log4rs::init_config(config)
        .expect("[Business Rule]: unable to init log4rs config");

    log::info!("logger is initialized");
}
