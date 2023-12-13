use tower_http::{
    classify::{
        ServerErrorsAsFailures,
        SharedClassifier,
    },
    trace::{
        DefaultMakeSpan,
        DefaultOnRequest,
        DefaultOnResponse,
        TraceLayer,
    },
    LatencyUnit,
};

pub type Layer = TraceLayer<SharedClassifier<ServerErrorsAsFailures>>;

pub fn init() {
    tracing_subscriber::fmt::init();
}

pub fn get_layer() -> Layer {
    // https://docs.rs/tower-http/0.1.1/tower_http/trace/index.html
    TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
        .on_response(
            DefaultOnResponse::new()
                .level(tracing::Level::INFO)
                .latency_unit(LatencyUnit::Micros),
        )
}
