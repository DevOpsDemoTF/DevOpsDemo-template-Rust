use super::*;
use actix_web::{web, HttpResponse};
use lazy_static::lazy_static;
use prometheus::{opts, register_counter, register_int_counter, IntCounter};

lazy_static! {
    static ref HEALTH_CHECK_COUNTER: IntCounter = register_int_counter!(
        "health_counter",
        "Number of times the health endpoint has been called"
    )
    .unwrap();
}

pub fn handler(data: web::Data<SharedState>) -> HttpResponse {
    HEALTH_CHECK_COUNTER.inc();
    let guard = data.read().unwrap();
    if guard.healthy {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
