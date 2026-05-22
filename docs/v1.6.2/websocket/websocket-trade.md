# 체결 (Trade)

체결 데이터를 WebSocket으로 수신하기 위한 요청 및 구독 데이터 예시를 제공합니다.

## Request 메세지 형식

체결 데이터 수신을 요청하기 위해서는 WebSocket 연결 이후 아래 구조의 JSON Object를 생성한 뒤 요청 메세지의 Data Type Object로 포함하여 전송해야 합니다. Ticket, Format 필드를 포함한 전체 WebSocket 데이터 요청 메세지 명세는 [WebSocket 사용 안내](https://docs.upbit.com/kr/reference/websocket-guide) 문서를 참고해주세요.

| 필드명 | 타입 | 내용 | 필수 여부 | 기본 값 |
|--------|------|------|-----------|---------|
| type | String | `trade` | Required | - |
| codes | List:String | 수신하고자 하는 페어 목록. 반드시 대문자로 요청해야 합니다. | Required | - |
| is_only_snapshot | Boolean | 스냅샷 시세만 제공 | Optional | `false` |
| is_only_realtime | Boolean | 실시간 시세만 제공 | Optional | `false` |

### 예시 - DEFAULT

```json
[
  {
    "ticket": "0e66c0ac-7e13-43ef-91fb-2a87c2956c49"
  },
  {
    "type": "trade",
    "codes": ["KRW-BTC","KRW-ETH"]
  },
  {
    "format": "DEFAULT"
  }
]
```

### 예시 - SIMPLE_LIST

```json
[
  {
    "ticket": "0e66c0ac-7e13-43ef-91fb-2a87c2956c49"
  },
  {
    "type": "trade",
    "codes": ["KRW-BTC","KRW-ETH"]
  },
  {
    "format": "SIMPLE_LIST"
  }
]
```

## 구독 데이터 명세

| 필드명 | 축약형 | 내용 | 타입 | 값 |
|--------|--------|------|------|-----|
| type | ty | 데이터 항목 | String | `trade` |
| code | cd | 페어 코드 | String | `KRW-BTC` |
| trade_price | tp | 체결 가격 | Double | - |
| trade_volume | tv | 체결량 | Double | - |
| ask_bid | ab | 매수/매도 구분 | String | `ASK` (매도), `BID` (매수) |
| prev_closing_price | pcp | 전일 종가 | Double | - |
| change | c | 전일 종가 대비 가격 변동 방향 | String | `RISE` (상승), `EVEN` (보합), `FALL` (하락) |
| change_price | cp | 전일 대비 가격 변동의 절대값 | Double | - |
| trade_date | td | 체결 일자(UTC 기준) | String | `yyyy-MM-dd` |
| trade_time | ttm | 체결 시각(UTC 기준) | String | `HH:mm:ss` |
| trade_timestamp | ttms | 체결 타임스탬프(ms) | Long | - |
| timestamp | tms | 타임스탬프(ms) | Long | - |
| sequential_id | sid | 체결 번호(Unique) | Long | - |
| best_ask_price | bap | 최우선 매도 호가 | Double | - |
| best_ask_size | bas | 최우선 매도 잔량 | Double | - |
| best_bid_price | bbp | 최우선 매수 호가 | Double | - |
| best_bid_size | bbs | 최우선 매수 잔량 | Double | - |
| stream_type | st | 스트림 타입 | String | `SNAPSHOT` (스냅샷), `REALTIME` (실시간) |

### 예시 - DEFAULT

```json
{
  "type": "trade",
  "code": "KRW-BTC",
  "timestamp": 1730336862082,
  "trade_date": "2024-10-31",
  "trade_time": "01:07:42",
  "trade_timestamp": 1730336862047,
  "trade_price": 100473000.00000000,
  "trade_volume": 0.00014208,
  "ask_bid": "BID",
  "prev_closing_price": 100571000.00000000,
  "change": "FALL",
  "change_price": 98000.00000000,
  "sequential_id": 17303368620470000,
  "best_ask_price": 100473000,
  "best_ask_size": 0.43139478,
  "best_bid_price": 100465000,
  "best_bid_size": 0.01990656,
  "stream_type": "SNAPSHOT"
}
{
  "type": "trade",
  "code": "KRW-ETH",
  "timestamp": 1730336862120,
  "trade_date": "2024-10-31",
  "trade_time": "01:07:42",
  "trade_timestamp": 1730336862080,
  "trade_price": 3700000.00000000,
  "trade_volume": 0.02207517,
  "ask_bid": "BID",
  "prev_closing_price": 3695000.00000000,
  "change": "RISE",
  "change_price": 5000.00000000,
  "sequential_id": 17303368620800006,
  "best_ask_price": 3700000,
  "best_ask_size": 0.39101775,
  "best_bid_price": 3699000,
  "best_bid_size": 0.13499454,
  "stream_type": "SNAPSHOT"
}
```

### 예시 - JSON_LIST

```json
[
  {
    "type": "trade",
    "code": "KRW-BTC",
    "timestamp": 1730336862082,
    "trade_date": "2024-10-31",
    "trade_time": "01:07:42",
    "trade_timestamp": 1730336862047,
    "trade_price": 100473000.00000000,
    "trade_volume": 0.00014208,
    "ask_bid": "BID",
    "prev_closing_price": 100571000.00000000,
    "change": "FALL",
    "change_price": 98000.00000000,
    "sequential_id": 17303368620470000,
    "best_ask_price": 100473000,
    "best_ask_size": 0.43139478,
    "best_bid_price": 100465000,
    "best_bid_size": 0.01990656,
    "stream_type": "SNAPSHOT"
  },
  {
    "type": "trade",
    "code": "KRW-ETH",
    "timestamp": 1730336862120,
    "trade_date": "2024-10-31",
    "trade_time": "01:07:42",
    "trade_timestamp": 1730336862080,
    "trade_price": 3700000.00000000,
    "trade_volume": 0.02207517,
    "ask_bid": "BID",
    "prev_closing_price": 3695000.00000000,
    "change": "RISE",
    "change_price": 5000.00000000,
    "sequential_id": 17303368620800006,
    "best_ask_price": 3700000,
    "best_ask_size": 0.39101775,
    "best_bid_price": 3699000,
    "best_bid_size": 0.13499454,
    "stream_type": "SNAPSHOT"
  }
]
```
