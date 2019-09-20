use slog_scope::info;

mod app;
mod config;
mod metrics;

fn main() {
    let config = config::new();
    metrics::init();

    info!("Service has been started");

    let app = app::new(&config);

    slog_scope::scope(
        &slog_scope::logger().new(slog::o!("scope" => "api")),
        || app.run("0.0.0.0:8080").unwrap(),
    );
}
