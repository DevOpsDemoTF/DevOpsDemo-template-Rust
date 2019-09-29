use actix_web::web;
use state::State;
use std::sync::{Arc, RwLock};

mod handlers;
mod state;

pub type SharedState = Arc<RwLock<state::State>>;

#[derive(Clone)]
pub struct Router {
    state: SharedState,
}

impl Router {
    pub fn new(config: &crate::Config) -> Router {
        Router {
            state: Arc::new(RwLock::new(State::new(config))),
        }
    }

    pub fn configure(self: &Router, app: &mut web::ServiceConfig) {
        app.data(self.state.clone())
            .route("/health", web::get().to(handlers::health_check::handler));
    }
}
