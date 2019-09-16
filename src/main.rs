#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

#[macro_use(slog_o)]
extern crate slog;

#[macro_use]
extern crate slog_scope;

#[macro_use]
extern crate prometheus;

mod app;
mod config;
mod handlers;
mod log;
mod metrics;

fn main() {
    let config = config::new();
    metrics::init();
    log::init(&config);

    info!("Service starting");

    let app = app::new(&config);

    slog_scope::scope(&slog_scope::logger().new(slog_o!("scope" => "api")), || {
        app.run("0.0.0.0:8080").unwrap()
    });
}
