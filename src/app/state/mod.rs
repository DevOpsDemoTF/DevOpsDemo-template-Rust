mod log;

pub struct State {
    pub healthy: bool,
    _logger_guard: slog_scope::GlobalLoggerGuard,
}

pub fn new(config: &crate::config::Config) -> State {
    State {
        healthy: true,
        _logger_guard: log::init(config),
    }
}
