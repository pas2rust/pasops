use neto::components::data::Http;
use neto::reqwest::header::{HeaderValue, USER_AGENT};
use std::io;
use std::sync::LazyLock;

use super::dto::MyResult;

static SHIELDS_HTTP: LazyLock<Result<Http, String>> = LazyLock::new(|| {
    let headers = vec![(USER_AGENT, HeaderValue::from_static("pasops-shields/1.0"))];

    let mut http = Http::new()
        .base_url("https://img.shields.io")
        .headers(headers)
        .build()
        .map_err(|e| e.to_string())?;

    http.config().map_err(|e| e.to_string())?;

    Ok(http)
});

pub fn shields_http() -> MyResult<&'static Http> {
    match &*SHIELDS_HTTP {
        Ok(h) => Ok(h),
        Err(s) => Err(Box::new(io::Error::other(s.clone()))),
    }
}

static CRATES_IO_HTTP: LazyLock<Result<Http, String>> = LazyLock::new(|| {
    let headers = vec![(
        USER_AGENT,
        HeaderValue::from_static("pasops-crates-downloads/1.0"),
    )];

    let mut http = Http::new()
        .base_url("https://crates.io/api/v1")
        .headers(headers)
        .build()
        .map_err(|e| e.to_string())?;

    http.config().map_err(|e| e.to_string())?;

    Ok(http)
});

pub fn crates_http() -> MyResult<&'static Http> {
    match &*CRATES_IO_HTTP {
        Ok(h) => Ok(h),
        Err(s) => Err(Box::new(io::Error::other(s.clone()))),
    }
}
