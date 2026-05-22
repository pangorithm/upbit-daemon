# 마켓 단위 현재가 조회

지정한 마켓(호가 자산) 내 모든 페어들의 현재가 정보를 조회합니다.

**Endpoint**: `GET https://api.upbit.com/v1/ticker/all`

**Rate Limit**: 초당 최대 10회 (IP 단위, ticker 그룹)

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `quote_currencies` | string | 필수 | 마켓 통화 코드 목록. 여러 통화는 쉼표 구분 (예: `KRW,BTC,USDT`) |

## 응답 필드

페어 단위 현재가 조회와 동일한 필드 구조. 지정한 마켓에 속한 모든 페어 정보 반환.

## 응답 예시

```json
[
  {
    "market": "KRW-BTC",
    "trade_date": "20250704",
    "trade_time": "051400",
    "trade_date_kst": "20250704",
    "trade_time_kst": "141400",
    "trade_timestamp": 1751606040365,
    "opening_price": 148737000,
    "high_price": 149360000,
    "low_price": 148288000,
    "trade_price": 148601000,
    "prev_closing_price": 148737000,
    "change": "FALL",
    "change_price": 136000,
    "change_rate": 0.0009143656,
    "signed_change_price": -136000,
    "signed_change_rate": -0.0009143656,
    "trade_volume": 0.00016823,
    "acc_trade_price": 31615925234.05438,
    "acc_trade_price_24h": 178448329314.96686,
    "acc_trade_volume": 212.38911576,
    "acc_trade_volume_24h": 1198.26954807,
    "highest_52_week_price": 163325000,
    "highest_52_week_date": "2025-01-20",
    "lowest_52_week_price": 72100000,
    "lowest_52_week_date": "2024-08-05",
    "timestamp": 1751606040403
  }
]
```

## 변경 이력

| 일자 | 변경 사항 |
|------|----------|
| 2024-09-04 | 마켓 단위 현재가 조회 기능 신규 지원 |
