# 현재가 (Ticker)

현재가 데이터를 WebSocket으로 수신하기 위한 요청 및 구독 데이터 예시를 제공합니다.

## Request 메세지 형식

현재가 데이터 수신을 요청하기 위해서는 WebSocket 연결 이후 아래 구조의 JSON Object를 생성한 뒤 요청 메세지의 Data Type Object로 포함하여 전송해야 합니다. Ticket, Format 필드를 포함한 전체 WebSocket 데이터 요청 메세지 명세는 [WebSocket 사용 안내](https://docs.upbit.com/kr/reference/websocket-guide) 문서를 참고해주세요.

| 필드명 | 타입 | 내용 | 필수 여부 | 기본 값 |
|--------|------|------|-----------|---------|
| type | String | `ticker` | Required | - |
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
    "type": "ticker",
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
    "type": "ticker",
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
| type | ty | 데이터 항목 | String | `ticker` |
| code | cd | 페어 코드 | String | `KRW-BTC` |
| opening_price | op | 시가 | Double | - |
| high_price | hp | 고가 | Double | - |
| low_price | lp | 저가 | Double | - |
| trade_price | tp | 현재가 | Double | - |
| prev_closing_price | pcp | 전일 종가 | Double | - |
| change | c | 전일 종가 대비 가격 변동 방향 | String | `RISE` (상승), `EVEN` (보합), `FALL` (하락) |
| change_price | cp | 전일 대비 가격 변동의 절대값 | Double | - |
| signed_change_price | scp | 전일 대비 가격 변동 값 | Double | - |
| change_rate | cr | 전일 대비 등락율의 절대값 | Double | - |
| signed_change_rate | scr | 전일 대비 등락율 | Double | - |
| trade_volume | tv | 가장 최근 거래량 | Double | - |
| acc_trade_volume | atv | 누적 거래량(UTC 0시 기준) | Double | - |
| acc_trade_volume_24h | atv24h | 24시간 누적 거래량 | Double | - |
| acc_trade_price | atp | 누적 거래대금(UTC 0시 기준) | Double | - |
| acc_trade_price_24h | atp24h | 24시간 누적 거래대금 | Double | - |
| trade_date | tdt | 최근 거래 일자(UTC) | String | `yyyyMMdd` |
| trade_time | ttm | 최근 거래 시각(UTC) | String | `HHmmss` |
| trade_timestamp | ttms | 체결 타임스탬프(ms) | Long | - |
| ask_bid | ab | 매수/매도 구분 | String | `ASK` (매도), `BID` (매수) |
| acc_ask_volume | aav | 누적 매도량 | Double | - |
| acc_bid_volume | abv | 누적 매수량 | Double | - |
| highest_52_week_price | h52wp | 52주 최고가 | Double | - |
| highest_52_week_date | h52wdt | 52주 최고가 달성일 | String | `yyyy-MM-dd` |
| lowest_52_week_price | l52wp | 52주 최저가 | Double | - |
| lowest_52_week_date | l52wdt | 52주 최저가 달성일 | String | `yyyy-MM-dd` |
| market_state | ms | 거래상태 | String | `PREVIEW` (입금지원), `ACTIVE` (거래지원가능), `DELISTED` (거래지원종료) |
| is_trading_suspended | its | 거래 정지 여부. Deprecated 필드로 참조 대상에서 제외하는 것을 권장합니다. | Boolean | - |
| delisting_date | dd | 거래지원 종료일 | Date | - |
| market_warning | mw | 유의 종목 여부. Deprecated 필드로 참조 대상에서 제외하는 것을 권장합니다. | String | `NONE` (해당없음), `CAUTION` (투자유의) |
| timestamp | tms | 타임스탬프 (ms) | Long | - |
| stream_type | st | 스트림 타입 | String | `SNAPSHOT` (스냅샷), `REALTIME` (실시간) |

### 예시 - DEFAULT

```json
{
  "type": "ticker",
  "code": "KRW-BTC",
  "opening_price": 31883000,
  "high_price": 32310000,
  "low_price": 31855000,
  "trade_price": 32287000,
  "prev_closing_price": 31883000.00000000,
  "acc_trade_price": 78039261076.51241000,
  "change": "RISE",
  "change_price": 404000.00000000,
  "signed_change_price": 404000.00000000,
  "change_rate": 0.0126713295,
  "signed_change_rate": 0.0126713295,
  "ask_bid": "ASK",
  "trade_volume": 0.03103806,
  "acc_trade_volume": 2429.58834336,
  "trade_date": "20230221",
  "trade_time": "074102",
  "trade_timestamp": 1676965262139,
  "acc_ask_volume": 1146.25573608,
  "acc_bid_volume": 1283.33260728,
  "highest_52_week_price": 57678000.00000000,
  "highest_52_week_date": "2022-03-28",
  "lowest_52_week_price": 20700000.00000000,
  "lowest_52_week_date": "2022-12-30",
  "market_state": "ACTIVE",
  "is_trading_suspended": false,
  "delisting_date": null,
  "market_warning": "NONE",
  "timestamp": 1676965262177,
  "acc_trade_price_24h": 228827082483.70729000,
  "acc_trade_volume_24h": 7158.80283560,
  "stream_type": "REALTIME"
}
```

### 예시 - SIMPLE_LIST

```json
[
  {
    "ty": "ticker",
    "cd": "KRW-BTC",
    "op": 148500000.0,
    "hp": 149064000.0,
    "lp": 148050000.0,
    "tp": 148956000.0,
    "pcp": 148500000.0,
    "atp": 16115868486.03173,
    "c": "RISE",
    "cp": 456000.0,
    "scp": 456000.0,
    "cr": 0.0030707071,
    "scr": 0.0030707071,
    "ab": "BID",
    "tv": 0.0602652,
    "atv": 108.51228784,
    "tdt": "20250707",
    "ttm": "022055",
    "ttms": 1751854855934,
    "aav": 36.9943567,
    "abv": 71.51793114,
    "h52wp": 163325000.0,
    "h52wdt": "2025-01-20",
    "st": "SNAPSHOT"
  }
]
```
