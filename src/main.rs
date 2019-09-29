use actix_web::{App, HttpServer};
use config::Config;
use slog_scope::info;

mod config;
mod metrics;
mod router;

fn main() {
    let config = Config::new();
    metrics::init();

    info!("Service has been started");

    slog_scope::scope(
        &slog_scope::logger().new(slog::o!("scope" => "api")),
        || {
            let router = router::Router::new(&config);
            let server = HttpServer::new(move || App::new().configure(|a| router.configure(a)))
                .bind("0.0.0.0:8080")
                .expect("Can not bind to port 8080");

            server.run().unwrap()
        },
    );
}
