#[macro_use]
extern crate serde_derive;

use http::status::StatusCode;
use std::sync::Mutex;
use tide::{error::ResultExt, response, App, Context, EndpointResult};

#[derive(Default)]
struct State {}

async fn handle_health_check(_cx: Context<State>) -> EndpointResult<String> {
    Ok("".to_owned())
}

fn main() {
    let mut app = App::with_state(State::default());
    app.at("/health").get(handle_health_check);

    app.run("0.0.0.0:8080").unwrap();
}
