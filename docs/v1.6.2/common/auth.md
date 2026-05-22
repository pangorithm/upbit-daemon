# 인증

REST API 및 WebSocket 사용을 위한 인증 가이드입니다.

거래 및 자산 관리(Exchange) REST API를 호출하거나 WebSocket 데이터를 수신하려면 API Key 기반의 인증이 반드시 필요합니다. 본 가이드를 통해 인증 방식을 확인하고 언어별 구현 예시를 참고 할 수 있습니다.

## API Key

### API Key 발급 및 구조 안내

인증 구현에 앞서 [API Key 발급 가이드](https://docs.upbit.com/kr/docs/api-key)를 참고해 API Key를 발급받고, 호출지의 IP를 허용 목록에 등록해야 합니다. API Key 당 최대 10개의 IP를 등록할 수 있습니다.

API Key는 Access Key와 Secret Key 쌍으로 구성되며, Secret Key는 발급 시에만 확인 가능합니다. Secret Key는 **외부에 노출되지 않도록 반드시 안전하게 보관**해야 하며, 토큰 생성 시 반드시 Access Key와 짝을 이루는 Secret Key를 사용해야 합니다.

### API Key의 권한 그룹

업비트 API Key는 발급 시 사용 용도에 따라 필요한 권한만 선택적으로 부여할 수 있습니다. API 호출 시 권한 오류가 발생했다면, 해당 API Key에 필요한 권한이 포함되어 있는지 확인하세요.

#### 권한 그룹별 허용 API

| 권한 그룹 | 허용 REST API | 허용 WebSocket 타입 |
|---------|--------------|-------------------|
| - | 서비스 - 통화별 입출금 서비스 상태 조회, 서비스 - API Key 목록 조회 | - |
| 자산조회 | 계정 잔고 조회 | 내 자산(MyAsset) 타입 데이터 구독 |
| 주문하기 | 주문 생성, 주문 생성 테스트, 단일 주문 취소, 지정 주문 목록 취소, 주문 일괄 취소, 취소 후 재주문 | - |
| 주문조회 | 주문 가능정보 조회, 단일 주문 조회, 주문 목록 조회, 체결 대기 주문 조회, 종료 주문 조회 | 내 주문 및 체결(MyOrder) 타입 데이터 구독 |
| 출금하기 | 디지털 자산 출금하기, 원화 출금하기, 디지털 자산 출금 취소 접수 | - |
| 출금조회 | 출금 가능 정보 조회, 출금 허용 주소 목록 조회, 단일 출금 조회, 출금 목록 조회 | - |
| 입금하기 | 원화 입금, 입금 UUID로 트래블룰 검증 요청, 입금 TXID로 트래블룰 검증 요청 | - |
| 입금 조회 | 입금 주소 생성 요청, 입금 가능 통화 조회, 단일 입금 주소 조회, 입금 주소 목록 조회, 단일 입금 조회, 입금 목록 조회, 트래블룰 지원 거래소 목록 조회 | - |

<br />

## 인증 토큰

인증 토큰이란 서버에 요청을 보낼 때 사용자의 신원 및 권한을 증명하기 위해 전달하는 문자열 정보입니다. 업비트 API는 [JWT 토큰](https://jwt.io) 기반 인증을 사용합니다.

### JWT 토큰 구조

JWT는 헤더(Header), 페이로드(Payload), 서명(Signature)의 세 부분으로 구성됩니다.

#### 헤더(Header)

```json
{
  "alg": "HS512",
  "typ": "JWT"
}
```

#### 페이로드(Payload)

| Key | 설명 | 필수 여부 |
|-----|------|----------|
| `access_key` | API Key의 Access key | 필수 |
| `nonce` | 무작위의 UUID 문자열. 매 요청마다 새로운 값 | 필수 |
| `query_hash` | 요청 쿼리 파라미터 또는 본문의 Hash값 | REST API 쿼리 파라미터/본문 있을 시 필수 |
| `query_hash_alg` | query_hash 생성시 사용한 Hash 알고리즘. 기본값 `SHA512` | 선택 |

```json
// 파라미터 또는 본문이 있는 REST API
{
  "access_key": "a7Xd92LmQW3vBtRzYpMj5CxNKeT1HuVs0fFgJcAw",
  "nonce": "b2f1e3f8-2dc1-4d6f-a838-c74c49b0e39a",
  "query_hash": "0b3e884d40cc992a85730cf470b4a3286f13d9c46204279ef32153bcdcd4edb7c12925e7266636e86a6d6ae5804a6bb394e632e4dba9b4045ad470c93720e5e6",
  "query_hash_alg": "SHA512"
}

// WebSocket 및 파라미터/본문 없는 REST API
{
  "access_key": "a7Xd92LmQW3vBtRzYpMj5CxNKeT1HuVs0fFgJcAw",
  "nonce": "b2f1e3f8-2dc1-4d6f-a838-c74c49b0e39a"
}
```

#### 서명(Signature)

API Key의 Secret Key를 사용하여 헤더와 페이로드를 서명합니다. HMAC-SHA512 기반입니다.

> **주의**: Secret Key는 Base64 인코딩 되어있지 않습니다. 별도의 Base64 디코딩을 수행할 필요가 없습니다.

### JWT 생성 가이드 - query_hash 값 생성

**GET 또는 DELETE REST API 요청 시**
- 실제 요청에 포함된 쿼리 문자열을 그대로 발췌하여 Hash합니다.
- URL 인코딩 되지 않은 쿼리 문자열을 기준으로 Hash 값을 생성합니다.

**POST REST API 요청 시**
- JSON 형식의 요청 본문의 모든 Key-Value 쌍을 쿼리 문자열 형식으로 가공 후 Hash합니다.
- 예: `{"market":"KRW-BTC","side":"bid"}` → `market=KRW-BTC&side=bid` → Hash

### 인증 토큰 전송 방식

- **Key**: `Authorization`
- **Value**: `Bearer {JWT 토큰}`

### 코드 예시

#### Python
```python
from urllib.parse import quote, unquote, urlencode
from typing import Any, Dict
import hashlib
import uuid
import jwt
import requests

def _build_query_string(params: Dict[str, Any]) -> str:
    return unquote(urlencode(params, doseq=True))

def _create_jwt(access_key: str, secret_key: str, query_string: str = "") -> str:
    payload = {"access_key": access_key, "nonce": str(uuid.uuid4())}
    if query_string:
        query_hash = hashlib.sha512(query_string.encode("utf-8")).hexdigest()
        payload["query_hash"] = query_hash
        payload["query_hash_alg"] = "SHA512"
    token = jwt.encode(payload, secret_key, algorithm="HS512")
    return token if isinstance(token, str) else token.decode('utf-8')
```

#### Java
```java
private static String createJwt(String accessKey, String secretKey, String queryString)
    throws NoSuchAlgorithmException {
    byte[] secretKeyBytes = secretKey.getBytes(StandardCharsets.UTF_8);
    Algorithm algorithm = Algorithm.HMAC512(secretKeyBytes);
    Arrays.fill(secretKeyBytes, (byte) 0);

    JWTCreator.Builder builder = JWT.create()
        .withHeader(Collections.singletonMap("alg", "HS512"))
        .withClaim("access_key", accessKey)
        .withClaim("nonce", UUID.randomUUID().toString());

    if (queryString != null && !queryString.isEmpty()) {
        builder.withClaim("query_hash", sha512(queryString));
        builder.withClaim("query_hash_alg", "SHA512");
    }
    return builder.sign(algorithm);
}
```

#### Node.js
```javascript
const jwt = require('jsonwebtoken');
const crypto = require('crypto');
const { v4: uuidv4 } = require('uuid');

function createJwt(accessKey, secretKey, queryString = '') {
  const payload = {
    access_key: accessKey,
    nonce: uuidv4(),
  };
  if (queryString) {
    payload.query_hash = crypto.createHash('sha512').update(queryString, 'utf8').digest('hex');
    payload.query_hash_alg = 'SHA512';
  }
  return jwt.sign(payload, secretKey, { algorithm: 'HS512' });
}
```

#### WebSocket 연결 요청 예시 (Python)
```python
import jwt
import uuid
import websocket

payload = {
    'access_key': "YOUR_ACCESS_KEY",
    'nonce': str(uuid.uuid4()),
}
jwt_token = jwt.encode(payload, "YOUR_SECRET_KEY");
headers = {"Authorization": f"Bearer {jwt_token}"}

ws_app = websocket.WebSocketApp("wss://api.upbit.com/websocket/v1/private",
                                header=headers,
                                on_message=on_message,
                                on_open=on_connect)
ws_app.run_forever()
```
