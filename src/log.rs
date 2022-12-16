use std::env;

use simplelog::*;

///初始化日志
pub fn init() {
    let log_config = ConfigBuilder::new()
        .set_time_format_rfc3339()
        .add_filter_ignore("sqlx".to_string())
        .add_filter_ignore_str("mio::poll")
        .add_filter_ignore_str("want")
        .set_thread_mode(ThreadLogMode::IDs)
        .set_thread_padding(ThreadPadding::Left(0))
        .build();
    let level;
    if let Ok(log_level) = env::var("lailai-log-level") {
        match log_level.as_str() {
            "trace" => level = LevelFilter::Trace,
            "debug" => level = LevelFilter::Debug,
            _ => level = LevelFilter::Info,
        }
    } else {
        level = LevelFilter::Info;
    }
    CombinedLogger::init(vec![TermLogger::new(
        level,
        log_config,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
}
