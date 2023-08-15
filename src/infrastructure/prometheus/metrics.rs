use once_cell::sync::Lazy;
use prometheus::{
    exponential_buckets,
    histogram_opts,
    opts,
    register_histogram,
    register_int_counter_vec,
    Histogram,
    IntCounterVec,
};

// Total requests
pub static HTTP_REQUESTS_TOTAL: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        opts!("http_requests_total", "Total number of HTTP requests"),
        &["method", "endpoint", "status"]
    )
    .unwrap()
});

// Request durations
pub static HTTP_REQUEST_DURATION: Lazy<Histogram> = Lazy::new(|| {
    register_histogram!(histogram_opts!(
        "http_request_duration_seconds",
        "Duration of HTTP requests",
        vec![0.05, 0.1, 0.5, 1.0, 2.0, 5.0, 10.0]
    ))
    .unwrap()
});

// Request size
pub static HTTP_REQUEST_SIZE: Lazy<Histogram> = Lazy::new(|| {
    register_histogram!(histogram_opts!(
        "http_request_size_bytes",
        "Size of HTTP requests",
        exponential_buckets(256.0, 2.0, 13).unwrap(),
        // vec![256.0, 512.0, 1024.0, 2048.0, 4096.0, 8192.0]
    ))
    .unwrap()
});

// Response size
pub static HTTP_RESPONSE_SIZE: Lazy<Histogram> = Lazy::new(|| {
    register_histogram!(histogram_opts!(
        "http_response_size_bytes",
        "Size of HTTP responses",
        exponential_buckets(256.0, 2.0, 13).unwrap(),
        // vec![256.0, 512.0, 1024.0, 2048.0, 4096.0, 8192.0]
    ))
    .unwrap()
});
