# REST API를 이용한 시세 조회

업비트 Quotation API를 이용하여 다음과 같은 시세 정보를 조회할 수 있습니다.

* **페어 목록**: 업비트에서 지원하는 모든 페어(거래 쌍) 목록
* **캔들(OHLCV)**: 시간 단위(초, 분, 일, 주, 월, 연)별 시가/고가/저가/종가/누적 거래량 등
* **체결(Trade)**: 특정 페어의 최근 체결 내역
* **현재가(Ticker)**: 최근 거래 시각, 시가/고가/저가/종가/전일 종가, 가격 변화, 거래량 등
* **호가(Orderbook)**: 현재 매수/매도 호가, 매수/매도 잔량 등

## 1. 페어 목록 조회 후 특정 페어의 현재가(Ticker) 스냅샷 조회

### 페어 목록 조회

```curl
curl --request GET \
  --url https://api.upbit.com/v1/market/all
```

응답:

```json
[
    {"market": "KRW-BTC", "korean_name": "비트코인", "english_name": "Bitcoin"},
    {"market": "KRW-POLYX", "korean_name": "폴리매쉬", "english_name": "Polymesh"}
]
```

### 현재가 조회

```curl
curl --request GET \
     --url 'https://api.upbit.com/v1/ticker?markets=KRW-BTC' \
     --header 'accept: application/json'
```

응답:

```json
[
  {
    "market": "KRW-BTC",
    "trade_date": "20250704",
    "opening_price": "148737000.00000000",
    "high_price": "149360000.00000000",
    "low_price": "148288000.00000000",
    "trade_price": "148601000.00000000",
    "prev_closing_price": "148737000.00000000",
    "change": "FALL",
    "change_price": 136000,
    "change_rate": 0.0009143656,
    "trade_volume": 0.00016823,
    "acc_trade_price_24h": 178448329314.96686,
    "acc_trade_volume_24h": 1198.26954807,
    "highest_52_week_price": 163325000,
    "highest_52_week_date": "2025-01-20",
    "lowest_52_week_price": 72100000,
    "lowest_52_week_date": "2024-08-05",
    "timestamp": 1751606040403
  }
]
```

## 2. 캔들 조회

### 최근 캔들 조회하기

```curl
curl --request GET \
  --url 'https://api.upbit.com/v1/candles/minutes/5?market=KRW-BTC&count=3'
```

응답:

```json
[
    {
        "market": "KRW-BTC",
        "candle_date_time_utc": "2025-08-01T14:00:00",
        "candle_date_time_kst": "2025-08-01T23:00:00",
        "opening_price": 159399000.00000000,
        "high_price": 159525000.00000000,
        "low_price": 159001000.00000000,
        "trade_price": 159090000.00000000,
        "candle_acc_trade_price": 3784659174.62006000,
        "candle_acc_trade_volume": 23.77492422,
        "unit": 5
    }
]
```

### to 파라미터로 특정 시간대의 캔들 조회하기

```
https://api.upbit.com/v1/candles/minutes/1?market=KRW-BTC&to=2025-07-27T07:00:00%2B09:00
```

> **캔들 생성 기준**: 캔들은 해당 시간대에 체결이 발생한 경우에만 생성됩니다. 비어있는 시간 구간을 고려한 구현이 필요합니다.

## 3. 시세 체결 조회

```curl
curl --request GET \
     --url 'https://api.upbit.com/v1/trades/ticks?market=KRW-BTC' \
     --header 'accept: application/json'
```

특정 일자의 체결 이력을 조회:

```curl
curl --request GET \
     --url 'https://api.upbit.com/v1/trades/ticks?market=KRW-BTC&days_ago=2' \
     --header 'accept: application/json'
```

응답:

```json
[
    {
        "market": "KRW-BTC",
        "trade_date_utc": "2025-07-30",
        "trade_time_utc": "21:59:59",
        "trade_price": 162806000.00000000,
        "trade_volume": 0.00012284,
        "prev_closing_price": 163158000.00000000,
        "change_price": -352000.00000000,
        "ask_bid": "BID",
        "sequential_id": 17539127999450000
    }
]
```

## 4. 호가 조회

```curl
curl --request GET \
     --url 'https://api.upbit.com/v1/orderbook?markets=KRW-BTC&count=2' \
     --header 'accept: application/json'
```

응답:

```json
[
    {
        "market": "KRW-BTC",
        "timestamp": 1754057310152,
        "total_ask_size": 1.44700033,
        "total_bid_size": 1.83572538,
        "orderbook_units": [
            {
                "ask_price": 159399000,
                "bid_price": 159396000,
                "ask_size": 0.50775343,
                "bid_size": 0.30813376
            },
            {
                "ask_price": 159400000,
                "bid_price": 159385000,
                "ask_size": 0.03768668,
                "bid_size": 0.0003137
            }
        ],
        "level": 0
    }
]
```
