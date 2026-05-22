# 캔들 (Candle)

캔들 데이터를 WebSocket으로 수신하기 위한 요청 및 구독 데이터 예시를 제공합니다.

## 캔들 실시간 스트림 전송 방식 안내

### 데이터 전송 주기
캔들 데이터의 실시간 스트림 전송 주기는 1초 입니다.

### 데이터 생성 안내
캔들은 해당 시간대에 체결이 발생하여 직전 캔들 대비 캔들 데이터가 변경될 때에만 생성됩니다. 전송 주기인 1초가 지나더라도 체결이 발생하지 않은 경우 실시간 캔들 데이터 스트림이 발생하지 않습니다. 또한 요청 시점에 요청한 단위의 캔들 데이터가 생성되지 않은 경우 이전 시간 단위의 데이터가 최초 전송됩니다.

3분 봉 요청 상황을 예로 들어, 12:00:00 3분 봉은 존재하고, 12:03:00 ~ 12:04:00 사이는 아직 체결이 없는 상태라고 가정합니다. 이때 12:04:00 에 `candle.3m` 요청을 보내면, 서버는 12:00:00 ~ 12:03:00 3분 봉 데이터를 스냅샷 데이터로 반환합니다. 이후 12:04:05 에 첫 체결이 발생하면 서버는 즉시 12:03:00 3분봉을 생성하고, 다음 1초 interval 인 12:04:06 에 해당 12:03:00 3분 봉 데이터를 전송하게 됩니다.

**⚠️ 같은 candle_date_time 데이터가 여러 번 전송될 수 있습니다.**

다양한 시간 단위의 요청을 처리하는 캔들 실시간 스트림의 특성상, 데이터의 전송 주기를 완벽하게 보장하기 어려우며 체결 타이밍에 따라 같은 시간대의 캔들 데이터가 여러번 전송될 수 있습니다. 가장 마지막으로 수신한 데이터가 최신 데이터이며, 사용 전 candle_date_time 필드를 참조하여 값을 업데이트 하시기 바랍니다.

## Request 메세지 형식

캔들 데이터 수신을 요청하기 위해서는 WebSocket 연결 이후 아래 구조의 JSON Object를 생성한 뒤 요청 메세지의 Data Type Object로 포함하여 전송해야 합니다. Ticket, Format 필드를 포함한 전체 WebSocket 데이터 요청 메세지 명세는 [WebSocket 사용 안내](https://docs.upbit.com/kr/reference/websocket-guide) 문서를 참고해주세요.

| 필드명 | 타입 | 내용 | 필수 여부 | 기본 값 |
|--------|------|------|-----------|---------|
| type | String | 캔들 형식<br>- `candle.1s`: 초봉<br>- `candle.1m`: 1분봉<br>- `candle.3m`: 3분봉<br>- `candle.5m`: 5분봉<br>- `candle.10m`: 10분봉<br>- `candle.15m`: 15분봉<br>- `candle.30m`: 30분봉<br>- `candle.60m`: 60분봉<br>- `candle.240m`: 240분봉 | Required | - |
| codes | List | 수신하고자 하는 페어 목록. 반드시 대문자로 요청해야 합니다. | Required | - |
| is_only_snapshot | Boolean | 스냅샷 시세만 제공 | Optional | `false` |
| is_only_realtime | Boolean | 실시간 시세만 제공 | Optional | `false` |

### 예시 - DEFAULT

```json
[
  {
    "ticket": "0e66c0ac-7e13-43ef-91fb-2a87c2956c49"
  },
  {
    "type": "candle.1s",
    "codes": ["KRW-BTC", "KRW-ETH"]
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
    "type": "candle.1s",
    "codes": ["KRW-BTC", "KRW-ETH"]
  },
  {
    "format": "SIMPLE_LIST"
  }
]
```

## 구독 데이터 명세

| 필드명 | 축약형 | 내용 | 타입 | 값 |
|--------|--------|------|------|-----|
| type | ty | 타입 | String | `candle.1s`, `candle.1m`, `candle.3m`, `candle.5m`, `candle.10m`, `candle.15m`, `candle.30m`, `candle.60m`, `candle.240m` |
| code | cd | 마켓 코드 (ex. KRW-BTC) | String | - |
| candle_date_time_utc | cdttmu | 캔들 기준 시각(UTC 기준)<br>포맷: `yyyy-MM-dd'T'HH:mm:ss` | String | - |
| candle_date_time_kst | cdttmk | 캔들 기준 시각(KST 기준)<br>포맷: `yyyy-MM-dd'T'HH:mm:ss` | String | - |
| opening_price | op | 시가 | Double | - |
| high_price | hp | 고가 | Double | - |
| low_price | lp | 저가 | Double | - |
| trade_price | tp | 종가 | Double | - |
| candle_acc_trade_volume | catv | 누적 거래량 | Double | - |
| candle_acc_trade_price | catp | 누적 거래 금액 | Double | - |
| timestamp | tms | 타임스탬프 (ms) | Long | - |
| stream_type | st | 스트림 타입 | String | `SNAPSHOT`: 스냅샷, `REALTIME`: 실시간 |

### 예시 - DEFAULT

```json
{
  "type": "candle.1s",
  "code": "KRW-BTC",
  "candle_date_time_utc": "2025-01-02T04:28:05",
  "candle_date_time_kst": "2025-01-02T13:28:05",
  "opening_price": 142009000.00000000,
  "high_price": 142009000.00000000,
  "low_price": 142009000.00000000,
  "trade_price": 142009000.00000000,
  "candle_acc_trade_volume": 0.00606119,
  "candle_acc_trade_price": 860743.5307100000000000,
  "timestamp": 1735792085824,
  "stream_type": "REALTIME"
}
```

```json
{
  "type": "candle.1s",
  "code": "KRW-ETH",
  "candle_date_time_utc": "2025-01-02T04:28:05",
  "candle_date_time_kst": "2025-01-02T13:28:05",
  "opening_price": 5059000.00000000,
  "high_price": 5059000.00000000,
  "low_price": 5059000.00000000,
  "trade_price": 5059000.00000000,
  "candle_acc_trade_volume": 0.08158869,
  "candle_acc_trade_price": 412757.1827100000000000,
  "timestamp": 1735792085749,
  "stream_type": "REALTIME"
}
```

### 예시 - SIMPLE_LIST

```json
[
  {
    "ty": "candle.1s",
    "cd": "KRW-BTC",
    "cdttmu": "2025-07-07T02:29:24",
    "cdttmk": "2025-07-07T11:29:24",
    "op": 149000000.0,
    "hp": 149000000.0,
    "lp": 149000000.0,
    "tp": 149000000.0,
    "catv": 0.00033557,
    "catp": 49999.93,
    "tms": 1751855364161,
    "st": "SNAPSHOT"
  },
  {
    "ty": "candle.1s",
    "cd": "KRW-ETH",
    "cdttmu": "2025-07-07T02:29:12",
    "cdttmk": "2025-07-07T11:29:12",
    "op": 3515000.0,
    "hp": 3515000.0,
    "lp": 3515000.0,
    "tp": 3515000.0,
    "catv": 0.01,
    "st": "SNAPSHOT"
  }
]
```
