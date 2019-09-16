use slog::Level;
use std::env;
use std::io::Write;
use std::str::FromStr;

pub struct Config {
    pub log_level: Level,
}

pub fn new() -> Config {
    Config {
        log_level: log_level(),
    }
}

fn log_level() -> Level {
    let level = env::var("LOG_LEVEL").unwrap_or_else(|_err| {
        writeln!(std::io::stderr(), "Missing LOG_LEVEL, defaulting to WARN",).unwrap();
        "WARN".to_string()
    });

    Level::from_str(level.as_str())
        .map_err(|err| {
            writeln!(
                std::io::stderr(),
                "Unknown log level \"{}\", defaulting to WARN",
                level.as_str(),
            )
            .unwrap();
            err
        })
        .unwrap_or(Level::Warning)
}
