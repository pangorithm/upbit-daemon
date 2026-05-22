use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use hmac::{Hmac, Mac};
use sha2::Sha512;
use uuid::Uuid;

type HmacSha512 = Hmac<Sha512>;

pub fn create_jwt(access_key: &str, secret_key: &str, query_string: Option<&str>) -> String {
    let mut payload = serde_json::Map::new();
    payload.insert(
        "access_key".to_string(),
        serde_json::Value::String(access_key.to_string()),
    );
    payload.insert(
        "nonce".to_string(),
        serde_json::Value::String(Uuid::new_v4().to_string()),
    );

    if let Some(qs) = query_string {
        let hash = sha512_hash(qs);
        payload.insert("query_hash".to_string(), serde_json::Value::String(hash));
        payload.insert(
            "query_hash_alg".to_string(),
            serde_json::Value::String("SHA512".to_string()),
        );
    }

    let header = serde_json::json!({"alg": "HS512", "typ": "JWT"});

    let header_b64 = url_safe_base64(&header.to_string());
    let payload_b64 = url_safe_base64(&serde_json::to_string(&payload).unwrap());

    let signature = hmac_sha512(&format!("{}.{}", header_b64, payload_b64), secret_key);
    let signature_b64 = URL_SAFE_NO_PAD.encode(signature);

    format!("{}.{}.{}", header_b64, payload_b64, signature_b64)
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

pub fn body_to_query_string(body: &serde_json::Value) -> String {
    if let Some(obj) = body.as_object() {
        let pairs: Vec<String> = obj
            .iter()
            .map(|(k, v)| {
                let val: String = v.as_str().map(String::from).unwrap_or(v.to_string());
                format!("{}={}", k, val)
            })
            .collect();
        pairs.join("&")
    } else {
        String::new()
    }
}

fn sha512_hash(s: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha512::new();
    hasher.update(s.as_bytes());
    hex::encode(hasher.finalize())
}

fn url_safe_base64(s: &str) -> String {
    URL_SAFE_NO_PAD.encode(s.as_bytes())
}

fn hmac_sha512(msg: &str, key: &str) -> Vec<u8> {
    let mut mac = HmacSha512::new_from_slice(key.as_bytes()).expect("HMAC key");
    mac.update(msg.as_bytes());
    mac.finalize().into_bytes().to_vec()
}
