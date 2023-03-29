pub fn init() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "╭[{} {} {}:{}]\n╰❯{}",
                chrono::Local::now().format("%H:%M:%S"),
                record.level(),
                record.file().unwrap_or("Unknown file"),
                // .replace("daemon\\src/", ""),
                record
                    .line()
                    .map(|l| l.to_string())
                    .unwrap_or("?".to_string()),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file(crate::config::LOG_FILE_LOCATION).unwrap())
        .apply()
        .unwrap();

    log_panics::Config::new()
        .backtrace_mode(log_panics::BacktraceMode::Resolved)
        .install_panic_hook()
}
