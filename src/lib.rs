use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub fn init_log(log_level: Option<&str>) {
    let mut builder = Builder::new();
    let level_filter = match log_level {
        Some("info") => LevelFilter::Info,
        Some("warn") => LevelFilter::Warn,
        Some("error") => LevelFilter::Error,
        _ => LevelFilter::Info,
    };
    builder
        .format(move |buf, record| {
            let style = buf.default_level_style(record.level());
            let level = style.value(record.level()); // 颜色受NO_COLOR这个环境变量控制
            let path = record.module_path().unwrap_or_default();
            writeln!(
                buf,
                "{} [{}] [{}]: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                level,
                path,
                record.args()
            )
        })
        .filter(None, level_filter)
        .init();
}
