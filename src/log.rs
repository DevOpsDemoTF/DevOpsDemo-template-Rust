extern crate chrono;
extern crate slog_json;
extern crate slog_stdlog;

use slog::{Drain, FnValue, PushFnValue, Record};
use std::sync::Mutex;

static mut _GUARD: Option<slog_scope::GlobalLoggerGuard> = None;
static mut _GUARD2: Option<()> = None;

pub fn init(config: &crate::config::Config) {
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
    unsafe {
        _GUARD = Some(slog_scope::set_global_logger(root));
        _GUARD2 = Some(slog_stdlog::init().unwrap());
    }
}
