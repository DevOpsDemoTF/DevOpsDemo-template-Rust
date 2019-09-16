use prometheus::{self, Encoder, TextEncoder};
use std::thread;
use tide::{App, Body, Context, EndpointResult};

pub async fn handle_metrics(_cx: Context<()>) -> EndpointResult {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    Ok(http::Response::builder()
        .status(http::status::StatusCode::OK)
        .header("Content-Type", encoder.format_type())
        .body(Body::from(buffer))
        .unwrap())
}

pub fn init() {
    let mut app = App::new();
    app.at("/metrics").get(handle_metrics);
    thread::spawn(move || {
        slog_scope::scope(
            &slog_scope::logger().new(slog_o!("scope" => "prometheus")),
            || app.run("0.0.0.0:9102").unwrap(),
        );
    });
}
