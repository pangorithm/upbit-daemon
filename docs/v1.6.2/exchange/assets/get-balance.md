# 계정 잔고 조회

계정이 보유하고 있는 자산 목록과 잔고를 조회합니다.

**Endpoint**: `GET https://api.upbit.com/v1/accounts`

**Rate Limit**: 초당 최대 30회 (계정 단위, exchange default 그룹)

**API Key Permission**: 자산조회 권한 필요

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `currency` | string | 통화 코드 (예: `BTC`) |
| `balance` | string | 주문 가능 수량/금액. 디지털 자산은 수량, KRW는 금액 |
| `locked` | string | 출금/주문 등에 잠겨 있는 잔액 |
| `avg_buy_price` | string | 매수 평균가 |
| `avg_buy_price_modified` | boolean | 매수 평균가 수정 여부 |
| `unit_currency` | string | 평균가 기준 통화 (`KRW`, `BTC`, `USDT` 등) |

## 응답 예시

```json
[
  {
    "currency": "KRW",
    "balance": "1000000.0",
    "locked": "0.0",
    "avg_buy_price": "0",
    "avg_buy_price_modified": false,
    "unit_currency": "KRW"
  },
  {
    "currency": "BTC",
    "balance": "2.0",
    "locked": "0.0",
    "avg_buy_price": "140000000",
    "avg_buy_price_modified": false,
    "unit_currency": "KRW"
  }
]
```
