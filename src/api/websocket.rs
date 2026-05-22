use super::auth;
use futures_util::{SinkExt, StreamExt};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{interval, Duration};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::Message;
use tracing::info;

type InnerStream = tokio_tungstenite::WebSocketStream<
    tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
>;

pub struct WebSocketClient {
    stream: Arc<Mutex<InnerStream>>,
    closed: Arc<AtomicBool>,
}

impl Clone for WebSocketClient {
    fn clone(&self) -> Self {
        Self {
            stream: Arc::clone(&self.stream),
            closed: Arc::clone(&self.closed),
        }
    }
}

impl WebSocketClient {
    pub async fn connect(
        ws_url: String,
        access_key: Option<String>,
        secret_key: Option<String>,
    ) -> Result<Self, crate::error::AppError> {
        let access_key = access_key.as_deref().unwrap_or("");
        let secret_key = secret_key.as_deref().unwrap_or("");
        let jwt = auth::create_ws_jwt(access_key, secret_key);
        let mut request = ws_url.into_client_request()?;
        request.headers_mut().insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", jwt).parse().unwrap(),
        );

        let (stream, _) = connect_async(request).await?;
        info!("WebSocket connected");
        Ok(Self {
            stream: Arc::new(Mutex::new(stream)),
            closed: Arc::new(AtomicBool::new(false)),
        })
    }

    pub async fn send(&self, msg: Message) -> Result<(), crate::error::AppError> {
        if self.closed.load(Ordering::Acquire) {
            return Err(crate::error::AppError::ConnectionClosed);
        }
        let mut stream = self.stream.lock().await;
        stream.send(msg).await?;
        Ok(())
    }

    pub async fn recv(&self) -> Option<Result<Message, crate::error::AppError>> {
        let mut stream = self.stream.lock().await;
        let result = stream.next().await;
        if result.is_none() {
            self.closed.store(true, Ordering::Release);
        }
        result.map(|r| r.map_err(crate::error::AppError::from))
    }

    pub fn keepalive(&self, tx: mpsc::Sender<()>) -> tokio::task::JoinHandle<()> {
        let stream = Arc::clone(&self.stream);
        let closed = Arc::clone(&self.closed);
        tokio::spawn(async move {
            let mut timer = interval(Duration::from_secs(55));
            loop {
                timer.tick().await;
                if closed.load(Ordering::Acquire) {
                    break;
                }
                let mut stream = stream.lock().await;
                if stream.send(Message::Ping(vec![].into())).await.is_err() {
                    let _ = tx.send(()).await;
                    break;
                }
            }
        })
    }
}
