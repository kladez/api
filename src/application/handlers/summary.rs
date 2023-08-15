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
    sleep,
    Duration,
};

use crate::{
    domain::services,
    infrastructure::Infrastructure,
};

#[handler]
pub fn summary(
    ws: WebSocket,
    Data(infrastructure): Data<&Infrastructure>,
) -> impl IntoResponse {
    let mut service = services::Summary::new(infrastructure);

    ws.on_upgrade(move |socket| async move {
        let (mut sink, mut stream) = socket.split();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    summary = service.get() => {
                        let message = match summary {
                            Ok(summary) => {
                                Message::Text(serde_json::to_string(&summary).unwrap())
                            },
                            Err(e) => {
                                tracing::error!(error = ?e, "getting summary error");
                                Message::Text(serde_json::json!({ "error": stringify!("getting summary error: {}", e) }).to_string())
                            }
                        };
                        if sink.send(message.clone()).await.is_err() {
                            break;
                        }
                        sleep(Duration::from_secs(1)).await;
                    }
                    None = stream.next() => {
                        break;
                    }
                }
            }
        });
    })
}
