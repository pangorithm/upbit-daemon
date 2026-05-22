# 내 주문 및 체결 (MyOrder)

내 주문 및 체결 데이터를 WebSocket으로 수신하기 위한 요청 및 구독 데이터 예시를 제공합니다.

## 내 주문 및 체결 실시간 스트림 전송 방식 안내

내 주문 및 체결 데이터의 경우 실제 주문 또는 체결이 발생할 때만 해당 내용이 실시간 스트림으로 전송됩니다. 따라서 **연결 후 주문 또는 체결이 발생하지 않는 경우 데이터 수신이 이루어지지 않는 것이 정상 동작**입니다. [WebSocket 사용 안내의 연결 유지 항목](https://docs.upbit.com/kr/reference/websocket-guide)을 참고하여 데이터 미전송시에도 연결이 유지되도록 클라이언트 사양 또는 구현을 확인하시기 바랍니다.

## Request 메세지 형식

내 주문 및 체결 데이터 수신을 요청하기 위해서는 WebSocket 연결 이후 아래 구조의 JSON Object를 생성한 뒤 요청 메세지의 Data Type Object로 포함하여 전송해야 합니다. Ticket, Format 필드를 포함한 전체 WebSocket 데이터 요청 메세지 명세는 [WebSocket 사용 안내](https://docs.upbit.com/kr/reference/websocket-guide) 문서를 참고해주세요.

| 필드명 | 타입 | 내용 | 필수 여부 | 기본 값 |
|--------|------|------|-----------|---------|
| type | String | `myOrder` | Required | - |
| codes | List | 수신하고자 하는 페어 목록. 반드시 대문자로 요청해야 합니다. | Optional | 생략하거나 빈 배열로 요청할 경우 모든 마켓에 대한 정보를 수신합니다. |

### 예시 - 모든 페어 구독

```json
[
  {
    "ticket": "0e66c0ac-7e13-43ef-91fb-2a87c2956c49"
  },
  {
    "type": "myOrder"
  }
]
```

또는

```json
[
  {
    "ticket": "test-myOrder"
  },
  {
    "type": "myOrder",
    "codes": []
  }
]
```

### 예시 - 특정 페어 구독

```json
[
  {
    "ticket": "0e66c0ac-7e13-43ef-91fb-2a87c2956c49"
  },
  {
    "type": "myOrder",
    "codes": ["KRW-BTC"]
  }
]
```

### 예시 - JSON LIST

```json
[
  {
    "ticket": "0e66c0ac-7e13-43ef-91fb-2a87c2956c49"
  },
  {
    "type": "myOrder",
    "codes": ["KRW-BTC"]
  },
  {
    "format": "JSON_LIST"
  }
]
```

## 구독 데이터 명세

**신규 필드 추가 안내 (2025.07.02)**

자전거래 체결 방지 (Self-Match Prevention) 기능 추가로 인해 주문 데이터 필드가 아래와 같이 추가됩니다. 자세한 사항은 관련 [공지](https://docs.upbit.com/kr/changelog/smp) 참고 부탁드립니다.
- `smp_type` : `reduce`, `cancel_maker`, `cancel_taker`
- 주문 상태 state 필드에 `prevented` (체결 방지) 타입이 신규로 추가됩니다.
- 신규 주문 조건 `post_only`가 추가되어 time_in_force 필드에 `post_only` 타입이 신규로 추가됩니다.

| 필드명 | 축약형 | 내용 | 타입 | 값 |
|--------|--------|------|------|-----|
| type | ty | 타입 | String | `myOrder` |
| code | cd | 페어 코드(예시:KRW-BTC) | String | - |
| uuid | uid | 주문의 유일 식별자 | String | - |
| ask_bid | ab | 매수/매도 구분 | String | `ASK` (매도), `BID` (매수) |
| order_type | ot | 주문 타입 | String | `limit` (지정가), `price` (시장가 매수), `market` (시장가 매도), `best` (최유리 지정가) |
| state | s | 주문 상태 | String | `wait` (체결 대기), `watch` (예약 주문 대기), `trade` (체결 발생), `done` (전체 체결 완료), `cancel` (주문 취소), `prevented` (체결 방지) |
| trade_uuid | tuid | 체결의 유일 식별자 | String | - |
| price | p | 주문 가격 또는 체결 가격(state: trade 일 때) | Double | - |
| avg_price | ap | 평균 체결 가격 | Double | - |
| volume | v | 주문량 또는 체결량 (state: trade 일 때) | Double | - |
| remaining_volume | rv | 체결 후 주문 잔량 | Double | - |
| executed_volume | ev | 체결된 수량 | Double | - |
| trades_count | tc | 해당 주문에 걸린 체결 수 | Integer | - |
| reserved_fee | rsf | 수수료로 예약된 비용 | Double | - |
| remaining_fee | rmf | 남은 수수료 | Double | - |
| paid_fee | pf | 사용된 수수료 | Double | - |
| locked | l | 거래에 사용중인 비용 | Double | - |
| executed_funds | ef | 체결된 금액 | Double | - |
| time_in_force | tif | IOC, FOK, POST ONLY 설정 | String | `ioc`, `fok`, `post_only` |
| trade_fee | tf | 체결 시 발생한 수수료<br>(state: `trade`가 아닐 경우 null) | Double | - |
| is_maker | im | 체결이 발생한 주문의 메이커/테이커 여부<br>(state: `trade`가 아닐 경우 null) | Boolean | `true`: 메이커, `false`: 테이커 |
| identifier | id | 클라이언트 지정 주문 식별자 | String | - |
| smp_type | smpt | 자전거래 체결 방지 타입<br>(동일 회원의 메이커/테이커 주문 간 체결 방지) | String | `reduce` (주문 줄이고 진행), `cancel_maker` (메이커 주문 취소), `cancel_taker` (테이커 주문 취소) |
| prevented_volume | pv | 자전거래 체결 방지로 인해 취소된 주문 수량 | Double | - |
| prevented_locked | pl | (매수 시)자전거래 체결 방지 설정으로 인해 취소된 금액<br>(매도 시)자전거래 체결 방지 설정으로 인해 취소된 수량 | Double | - |
| trade_timestamp | ttms | 체결 타임스탬프 (ms) | Long | - |
| order_timestamp | otms | 주문 타임스탬프 (ms) | Long | - |
| timestamp | tms | 타임스탬프 (ms) | Long | - |
| stream_type | st | 스트림 타입 | String | `REALTIME` (실시간), `SNAPSHOT` (스냅샷) |

### 예시 - DEFAULT

```json
{
  "type": "myOrder",
  "code": "KRW-BTC",
  "uuid": "ac2dc2a3-fce9-40a2-a4f6-5987c25c438f",
  "ask_bid": "BID",
  "order_type": "limit",
  "state": "trade",
  "trade_uuid": "68315169-fba4-4175-ade3-aff14a616657",
  "price": 0.001453,
  "avg_price": 0.00145372,
  "volume": 30925891.29839369,
  "remaining_volume": 29968038.09235948,
  "executed_volume": 30925891.29839369,
  "trades_count": 1,
  "reserved_fee": 44.23943970238218,
  "remaining_fee": 21.77177967409916,
  "paid_fee": 22.467660028283017,
  "locked": 43565.33112787242,
  "executed_funds": 44935.32005656603,
  "time_in_force": null,
  "trade_fee": 22.467660028283017,
  "is_maker": true,
  "identifier": "test-1",
  "smp_type": "cancel_maker",
  "prevented_volume": 1.174291929,
  "prevented_locked": 0.001706246173,
  "trade_timestamp": 1710751590421,
  "order_timestamp": 1710751590000,
  "timestamp": 1710751597500,
  "stream_type": "REALTIME"
}
```

### 예시 - JSON_LIST

```json
[
 {
    "type": "myOrder",
    "code": "KRW-BTC",
    "uuid": "ac2dc2a3-fce9-40a2-a4f6-5987c25c438f",
    "ask_bid": "BID",
    "order_type": "limit",
    "state": "trade",
    "trade_uuid": "68315169-fba4-4175-ade3-aff14a616657",
    "price": 0.001453,
    "avg_price": 0.00145372,
    "volume": 30925891.29839369,
    "remaining_volume": 29968038.09235948,
    "executed_volume": 30925891.29839369,
    "trades_count": 1,
    "reserved_fee": 44.23943970238218,
    "remaining_fee": 21.77177967409916,
    "paid_fee": 22.467660028283017,
    "locked": 43565.33112787242,
    "executed_funds": 44935.32005656603,
    "time_in_force": null,
    "trade_fee": 22.467660028283017,
    "is_maker": true,
    "identifier": "test-1",
    "smp_type": "cancel_maker",
    "prevented_volume": 1.174291929,
    "prevented_locked": 0.001706246173,
    "trade_timestamp": 1710751590421,
    "order_timestamp": 1710751590000,
    "timestamp": 1710751597500,
    "stream_type": "REALTIME"
  }
]
```
