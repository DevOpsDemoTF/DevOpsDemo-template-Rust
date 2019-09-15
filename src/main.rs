#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate prometheus;

mod app;
mod handlers;
mod metrics;

fn main() {
    metrics::init();
    let app = app::new();
    app.run("0.0.0.0:8080").unwrap();
}
