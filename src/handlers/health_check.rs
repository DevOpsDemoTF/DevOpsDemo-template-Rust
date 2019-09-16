use prometheus::IntCounter;
use tide::{Context, EndpointResult};

lazy_static! {
    static ref HEALTH_CHECK_COUNTER: IntCounter = register_int_counter!(
        "health_counter",
        "Number of times the health endpoint has been called"
    )
    .unwrap();
}

pub async fn handle_health_check(_cx: Context<crate::app::State>) -> EndpointResult<String> {
    HEALTH_CHECK_COUNTER.inc();
    Ok("".to_string())
}
