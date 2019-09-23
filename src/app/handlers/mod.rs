use http::StatusCode;
use tide::{Body, Error};

pub mod health_check;

type SharedState = super::SharedState;

#[cfg(test)]
mod tests;

fn wrap_err<E: std::error::Error>(err: E) -> Error {
    let resp = http::Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(err.to_string()))
        .unwrap();
    resp.into()
}
