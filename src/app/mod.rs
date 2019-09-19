use std::sync::RwLock;
use tide::App;

mod handlers;
mod state;
pub type SharedState = RwLock<state::State>;

pub fn new(config: &crate::config::Config) -> App<SharedState> {
    let state = state::new(config);
    let mut app = App::with_state(RwLock::new(state));
    app.at("/health").get(handlers::health_check::handler);
    app
}
