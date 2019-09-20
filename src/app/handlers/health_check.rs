use http::StatusCode;
use lazy_static::lazy_static;
use prometheus::{opts, register_counter, register_int_counter, IntCounter};
use tide::{Context, EndpointResult};

lazy_static! {
    static ref HEALTH_CHECK_COUNTER: IntCounter = register_int_counter!(
        "health_counter",
        "Number of times the health endpoint has been called"
    )
    .unwrap();
}

pub async fn handler(cx: Context<super::SharedState>) -> EndpointResult<String> {
    HEALTH_CHECK_COUNTER.inc();
    let state = cx.state().read().unwrap();
    if state.healthy {
        Ok("".to_string())
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR.into())
    }
}
