# WebSocket 사용 및 에러 안내

업비트 WebSocket 연결 및 데이터 수신을 위한 사용 안내입니다.

## Endpoint

| 구분 | Endpoint |
|-----|----------|
| **시세(Quotation)** | `wss://api.upbit.com/websocket/v1` |
| **자산 및 주문(Exchange)** | `wss://api.upbit.com/websocket/v1/private` |

## TLS

**TLS 1.2 이상** 지원. TLS 1.3 권장.

## 인증

Exchange 데이터 수신 시 JWT 토큰을 Authorization 헤더에 포함:

```
Authorization: Bearer eyJhb...d8sTw
```

> **주의**: 일부 WebSocket 클라이언트(wscat 등)는 커스텀 헤더 설정을 지원하지 않으므로 Exchange 데이터 수신 확인이 어려울 수 있습니다.

## 요청 수 제한

[요청 수 제한(Rate Limits)](./rate-limits.md) 문서 참조.

## 에러 안내

### 에러 응답 형식

```json
{
  "error": {
    "name": "ERROR_CODE",
    "message": "ERROR_MESSAGE"
  }
}
```

### 주요 에러 코드

| error.name | 발생 이유 |
|-----------|----------|
| `INVALID_AUTH` | 인증 정보 누락 또는 인증 토큰 검증 실패 |
| `WRONG_FORMAT` | 요청 형식 위반 |
| `NO_TICKET` | 티켓 필드 누락 |
| `NO_TYPE` | 타입 필드 누락 |
| `NO_CODES` | 코드 필드 누락 |
| `INVALID_PARAM` | 필수 요청 파라미터 누락 또는 지원하지 않는 값 |

## 데이터 항목

| 데이터 항목 (type) | 설명 | 지원 형식 |
|-------------------|------|----------|
| `ticker` | 현재가 데이터 수신 | 스냅샷, 실시간 스트림 |
| `trade` | 체결 데이터 수신 | 스냅샷, 실시간 스트림 |
| `orderbook` | 호가 데이터 수신 | 스냅샷, 실시간 스트림 |
| `candle.{unit}` | 캔들(초봉, 분봉) 데이터 수신 | 스냅샷, 실시간 스트림 |
| `myAsset` | 내 자산 데이터 수신 | 실시간 스트림 |
| `myOrder` | 내 주문 데이터 수신 | 실시간 스트림 |

## 데이터 유형

- **스냅샷**: 요청 시점의 정보를 1회 수신
- **실시간 스트림**: 연결 유지 중 지속적으로 정보 수신

## 요청 메세지 구조

JSON Array 형식, 다음 Object들을 포함:

### 1. Ticket Object (첫 번째 요소)

| 필드명 | 형식 | 필수 | 설명 |
|-------|------|------|------|
| `ticket` | String | Required | 요청 티켓 고유 식별자 (UUID 등) |

### 2. Data Type Object (두 번째 요소부터)

| 필드명 | 형식 | 필수 | 설명 |
|-------|------|------|------|
| `type` | String | Required | 데이터 항목 (`ticker`, `trade`, `orderbook`, `candle.{unit}`, `myAsset`, `myOrder`) |
| `codes` | String | Conditional | 조회 페어 목록. ticker/trade/orderbook/candle 필수 |
| `level` | String | Optional | 호가 모아보기 단위 (orderbook 전용) |
| `is_only_snapshot` | Boolean | Optional | 스냅샷만 요청 |
| `is_only_realtime` | Boolean | Optional | 실시간 스트림만 요청 |

### 3. Format Object (마지막 요소)

| 필드명 | 형식 | 필수 | 설명 |
|-------|------|------|------|
| `format` | String | Required | `DEFAULT`, `SIMPLE`, `JSON_LIST`, `SIMPLE_LIST` |

### 요청 예제

```json
// 단일 Ticker 스냅샷/실시간 스트림
[
  {"ticket":"3e2c4a9f-f0a7-457f-945e-4b57bde9f1ec"},
  {"type":"ticker","codes":["KRW-BTC"]}
]

// Trade, Orderbook SIMPLE 포맷
[
  {"ticket":"9a65cd93-8786-4202-9b13-bd90e0c8b64b"},
  {"type":"trade","codes":["KRW-BTC","BTC-BCH"]},
  {"type":"orderbook","codes":["KRW-BTC","BTC-BCH"]},
  {"format":"SIMPLE"}
]
```

## 연결 관리

- 서버는 120초 Idle Timeout으로 연결 종료
- 클라이언트는 주기적으로 PING Frame 보내 연결 유지
- "PING" 메시지 송신 시 서버가 10초 간격 `{"status":"UP"}` 응답

## 압축

[RFC 7692](https://tools.ietf.org/html/rfc7692) Compression 지원. 클라이언트 라이브러리에서 압축 옵션 활성화 가능.

## 연결 및 요청 테스트

```shell
# wscat 사용
npm install -g wscat
wscat -c wss://api.upbit.com/websocket/v1

# telsocket 사용
telsocket -url wss://api.upbit.com/websocket/v1
```
