use actix_web::{web, App, HttpResponse, HttpServer};
use prometheus::{self, Encoder, TextEncoder};
use std::thread;

fn handle_metrics() -> HttpResponse {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    HttpResponse::Ok()
        .content_type(encoder.format_type())
        .body(buffer)
}

pub fn init() {
    thread::spawn(|| {
        let server =
            HttpServer::new(|| App::new().route("/metrics", web::get().to(handle_metrics)))
                .bind("0.0.0.0:9102")
                .expect("Can not bind to port 9102");

        slog_scope::scope(
            &slog_scope::logger().new(slog::o!("scope" => "prometheus")),
            || server.run().unwrap(),
        );
    });
}
