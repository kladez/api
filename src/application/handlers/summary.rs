use futures_util::{
    SinkExt,
    StreamExt,
};
use poem::{
    handler,
    web::{
        websocket::{
            Message,
            WebSocket,
        },
        Data,
    },
    IntoResponse,
};
use tokio::time::{
    interval,
    Duration,
};
use tokio_stream::wrappers::IntervalStream;

use crate::{
    application::dtos,
    domain::services,
    infrastructure::Infrastructure,
};

fn create_message(summary_result: Result<dtos::summary::Summary, dtos::Error>) -> Message {
    match summary_result {
        Ok(summary) => {
            Message::Text(serde_json::to_string(&summary).expect("Serialization failed"))
        }
        Err(e) => {
            tracing::error!(error = ?e, "getting summary error");
            Message::Text(
                serde_json::json!({ "error": format!("getting summary error: {:?}", e) })
                    .to_string(),
            )
        }
    }
}

async fn handle_connection(
    mut sink: impl SinkExt<Message> + Unpin,
    mut stream: impl StreamExt<Item = Result<Message, std::io::Error>> + Unpin,
    mut service: services::Summary,
) {
    let mut interval = IntervalStream::new(interval(Duration::from_secs(1)));

    loop {
        tokio::select! {
            _ = interval.next() => {
                let message = create_message(service.get().await);
                if sink.send(message).await.is_err() {
                    break;
                }
            },
            next = stream.next() => {
                if next.is_none() || next.unwrap().is_err() {
                    break;
                }
            }
        }
    }
}

#[handler]
pub fn handler(
    ws: WebSocket,
    Data(infrastructure): Data<&Infrastructure>,
) -> impl IntoResponse {
    let service = services::Summary::new(infrastructure);

    ws.on_upgrade(move |socket| {
        let (sink, stream) = socket.split();
        handle_connection(sink, stream, service)
    })
}
