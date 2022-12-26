use std::env;

use tracing::{metadata::LevelFilter, Level};
use tracing_subscriber::{filter, fmt, prelude::*};

///初始化日志
pub fn init() {
    let level;
    if let Ok(log_level) = env::var("lailai-log-level") {
        match log_level.as_str() {
            "trace" => level = Level::TRACE,
            "debug" => level = Level::DEBUG,
            _ => level = Level::INFO,
        }
    } else {
        level = Level::INFO;
    };
    let debug = cfg!(debug_assertions);
    let layer = fmt::layer()
        .pretty()
        .with_file(debug)
        .with_line_number(debug)
        .with_filter(LevelFilter::from_level(level))
        .with_filter(filter::filter_fn(|m| {
            m.target().contains("lailai")
                || m.target().contains("ricq")
                || m.target().contains("fflogsv1")
        }));
    tracing_subscriber::registry().with(layer).init();
}
