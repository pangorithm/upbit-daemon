use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use uuid::Uuid;

pub fn create_jwt(access_key: &str, secret_key: &str, query_string: Option<&str>) -> String {
    let mut payload = serde_json::Map::new();
    payload.insert("access_key".to_string(), serde_json::Value::String(access_key.to_string()));
    payload.insert("nonce".to_string(), serde_json::Value::String(Uuid::new_v4().to_string()));

    if let Some(qs) = query_string {
        use sha2::{Digest, Sha512};
        let hash = Sha512::digest(qs.as_bytes());
        payload.insert("query_hash".to_string(), serde_json::Value::String(hex::encode(hash)));
        payload.insert(
            "query_hash_alg".to_string(),
            serde_json::Value::String("SHA512".to_string()),
        );
    }

    let header = Header {
        alg: Algorithm::HS512,
        ..Default::default()
    };
    encode(&header, &payload, &EncodingKey::from_secret(secret_key.as_bytes())).unwrap()
}

pub fn create_ws_jwt(access_key: &str, secret_key: &str) -> String {
    create_jwt(access_key, secret_key, None)
}

pub fn query_string(params: &[(&str, &str)]) -> String {
    params
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&")
}

#[allow(dead_code)]
pub fn body_to_query_string(body: &serde_json::Value) -> String {
    if let Some(obj) = body.as_object() {
        obj.iter()
            .map(|(k, v)| {
                let val: String = v.as_str().map(String::from).unwrap_or(v.to_string());
                format!("{}={}", k, val)
            })
            .collect::<Vec<_>>()
            .join("&")
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::{SinkExt, StreamExt};
    use tokio_tungstenite::tungstenite::client::IntoClientRequest;
    use tokio_tungstenite::tungstenite::Message;

    #[test]
    fn test_query_string_for_hash() {
        let params = vec![("market", "KRW-BTC"), ("side", "bid")];
        let qs = query_string(&params);
        assert_eq!(qs, "market=KRW-BTC&side=bid");
    }

    #[test]
    fn test_body_to_query_string() {
        let body = serde_json::json!({"market": "KRW-BTC", "side": "bid"});
        let qs = body_to_query_string(&body);
        assert_eq!(qs, "market=KRW-BTC&side=bid");
    }

    #[test]
    fn test_jwt_structure() {
        let jwt = create_jwt("test_access_key", "test_secret_key", Some("market=KRW-BTC&side=bid"));
        let parts: Vec<&str> = jwt.split('.').collect();
        assert_eq!(parts.len(), 3);

        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
        let payload_bytes = URL_SAFE_NO_PAD.decode(parts[1]).unwrap();
        let payload = std::str::from_utf8(&payload_bytes).unwrap();
        let payload_val: serde_json::Value = serde_json::from_str(payload).unwrap();

        assert_eq!(payload_val["access_key"], "test_access_key");
        assert!(payload_val["nonce"].is_string());
        assert!(payload_val["query_hash"].is_string());
        assert_eq!(payload_val["query_hash_alg"], "SHA512");

        use sha2::{Digest, Sha512};
        let expected_hash = hex::encode(Sha512::digest("market=KRW-BTC&side=bid"));
        assert_eq!(payload_val["query_hash"], expected_hash);
    }

    #[test]
    fn test_ws_jwt_no_query_hash() {
        let jwt = create_ws_jwt("test_access_key", "test_secret_key");
        let parts: Vec<&str> = jwt.split('.').collect();

        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
        let payload_bytes = URL_SAFE_NO_PAD.decode(parts[1]).unwrap();
        let payload = std::str::from_utf8(&payload_bytes).unwrap();
        let payload_val: serde_json::Value = serde_json::from_str(payload).unwrap();

        assert!(!payload_val.as_object().unwrap().contains_key("query_hash"));
        assert!(!payload_val.as_object().unwrap().contains_key("query_hash_alg"));
        assert_eq!(payload_val["access_key"], "test_access_key");
        assert!(payload_val["nonce"].is_string());
    }

    #[test]
    #[ignore = "requires .env with UPBIT_ACCESS_KEY and UPBIT_SECRET_KEY"]
    fn test_jwt_with_real_api() {
        if let Ok(contents) = std::fs::read_to_string(".env") {
            for line in contents.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') { continue; }
                if let Some((k, v)) = line.split_once('=') {
                    std::env::set_var(k.trim(), v.trim());
                }
            }
        }
        let access_key = std::env::var("UPBIT_ACCESS_KEY").expect("UPBIT_ACCESS_KEY not set");
        let secret_key = std::env::var("UPBIT_SECRET_KEY").expect("UPBIT_SECRET_KEY not set");

        let jwt = create_jwt(&access_key, &secret_key, None);

        let resp = reqwest::blocking::Client::new()
            .get("https://api.upbit.com/v1/accounts")
            .bearer_auth(&jwt)
            .send()
            .expect("API call failed");

        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().expect("Invalid JSON response");
        assert!(body.as_array().expect("Expected array").len() > 0);
    }

    #[tokio::test]
    #[ignore = "requires .env with UPBIT_ACCESS_KEY and UPBIT_SECRET_KEY"]
    async fn test_ws_jwt_with_real_api() {
        if let Ok(contents) = std::fs::read_to_string(".env") {
            for line in contents.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') { continue; }
                if let Some((k, v)) = line.split_once('=') {
                    std::env::set_var(k.trim(), v.trim());
                }
            }
        }
        let access_key = std::env::var("UPBIT_ACCESS_KEY").expect("UPBIT_ACCESS_KEY not set");
        let secret_key = std::env::var("UPBIT_SECRET_KEY").expect("UPBIT_SECRET_KEY not set");

        let jwt = create_ws_jwt(&access_key, &secret_key);
        assert!(jwt.len() > 50);

        let mut request = "wss://api.upbit.com/websocket/v1/private"
            .into_client_request()
            .expect("Invalid request");
        request.headers_mut().insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", jwt).parse().unwrap(),
        );

        let (mut stream, _) = tokio_tungstenite::connect_async(request).await.expect("WebSocket connect failed");

        let query = serde_json::json!([
            {
                "type": "MyAsset",
                "currencies": ["KRW", "BTC"]
            }
        ]);
        stream.send(Message::Text(query.to_string().into())).await.expect("Send failed");

        loop {
            let msg = stream.next().await.expect("Stream ended");
            let msg = msg.expect("Receive failed");
            match msg {
                Message::Text(text) => {
                    let body: serde_json::Value = serde_json::from_str(&text).expect("Invalid JSON");
                    assert!(body.is_array(), "Expected array response");
                    assert!(body.as_array().unwrap().len() > 0, "Expected non-empty array");
                    println!("Response: {}", text);
                    break;
                }
                Message::Ping(_) | Message::Pong(_) => continue,
                Message::Close(_) => break,
                _ => continue,
            }
        }
    }
}
