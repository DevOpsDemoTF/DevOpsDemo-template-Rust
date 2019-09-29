mod log;

pub struct State {
    pub healthy: bool,
    _logger_guard: slog_scope::GlobalLoggerGuard,
}

impl State {
    pub fn new(config: &crate::config::Config) -> State {
        State {
            healthy: true,
            _logger_guard: log::init(config),
        }
    }
}
