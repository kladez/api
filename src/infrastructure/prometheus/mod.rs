use poem::{
    async_trait,
    handler,
    Endpoint,
    IntoResponse,
    Middleware,
    Request,
    Response,
    Result,
};
use poem_openapi::payload::PlainText;
use prometheus::Encoder;

mod metrics;

use metrics::*;

#[derive(Debug)]
pub struct PrometheusMetrics;

impl<E: Endpoint> Middleware<E> for PrometheusMetrics {
    type Output = PrometheusMetricsImpl<E>;

    fn transform(
        &self,
        ep: E,
    ) -> Self::Output {
        PrometheusMetricsImpl(ep)
    }
}

#[derive(Debug)]
pub struct PrometheusMetricsImpl<E>(E);

#[async_trait]
impl<E: Endpoint> Endpoint for PrometheusMetricsImpl<E> {
    type Output = Response;

    async fn call(
        &self,
        req: Request,
    ) -> Result<Self::Output> {
        let start_time = std::time::Instant::now();

        let method = req.method().as_str().to_owned();
        let endpoint = req.uri().path().to_owned();

        // Record the request size
        if let Some(size) = req.headers().get("content-length") {
            if let Ok(size) = size.to_str().unwrap_or("").parse::<f64>() {
                HTTP_REQUEST_SIZE.observe(size);
            }
        }

        let res = self.0.call(req).await;
        let duration = start_time.elapsed();

        // Record the duration of the request
        HTTP_REQUEST_DURATION.observe(duration.as_secs_f64());

        let res = res.map(|res| res.into_response());

        let status = match &res {
            Ok(res) => res.status().as_str().to_owned(),
            Err(_) => "999".into(),
        };

        // Record the response size
        if let Some(size) = res
            .as_ref()
            .ok()
            .and_then(|res| res.headers().get("content-length"))
        {
            if let Ok(size) = size.to_str().unwrap_or("").parse::<f64>() {
                HTTP_RESPONSE_SIZE.observe(size);
            }
        }

        // Record the total requests
        HTTP_REQUESTS_TOTAL
            .with_label_values(&[&method, &endpoint, &status])
            .inc();

        res
    }
}

#[handler]
pub fn prometheus_metrics() -> PlainText<String> {
    let mut buffer = vec![];
    let encoder = prometheus::TextEncoder::new();
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    PlainText(String::from_utf8(buffer).unwrap())
}
