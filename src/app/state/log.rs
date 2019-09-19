extern crate chrono;
extern crate slog_json;
extern crate slog_stdlog;

use slog::{Drain, FnValue, PushFnValue, Record};
use std::sync::Mutex;

pub fn init(config: &crate::config::Config) -> slog_scope::GlobalLoggerGuard {
    let drain = slog_json::Json::new(std::io::stderr())
        .add_key_value(slog_o!(
        "time" => slog::PushFnValue(move |_ : &Record, ser| {
            ser.emit(chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true))
        })))
        .add_key_value(slog_o!(
        "level" => FnValue(move |rinfo : &Record| {
            rinfo.level().as_str()
        })))
        .add_key_value(slog_o!(
        "msg" => PushFnValue(move |record : &Record, ser| {
            ser.emit(record.msg())
        })))
        .build();

    let drain = slog::LevelFilter::new(drain, config.log_level);
    let root = slog::Logger::root(Mutex::new(drain).map(slog::Fuse), slog_o!());
    let guard = slog_scope::set_global_logger(root);
    slog_stdlog::init().unwrap();

    return guard;
}
