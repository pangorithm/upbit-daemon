use super::auth;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::Message;
use tracing::info;

pub struct WebSocketClient {
    stream: tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
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
        Ok(Self { stream })
    }

    pub async fn send(&mut self, msg: Message) -> Result<(), crate::error::AppError> {
        self.stream.send(msg).await?;
        Ok(())
    }

    pub async fn recv(&mut self) -> Option<Result<Message, crate::error::AppError>> {
        self.stream
            .next()
            .await
            .map(|r| r.map_err(crate::error::AppError::from))
    }
}
