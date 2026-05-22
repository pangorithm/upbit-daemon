# 호가 정책 조회

지정한 페어들의 호가 단위(tick_size)와 호가 모아보기 단위(supported_levels) 정보를 조회합니다.

**Endpoint**: `GET https://api.upbit.com/v1/orderbook/instruments`

**Rate Limit**: 초당 최대 10회 (IP 단위, orderbook 그룹)

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `markets` | string | 필수 | 페어 목록. 여러 페어는 쉼표 구분 |

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `market` | string | 페어 코드 |
| `quote_currency` | string | 마켓 통화 코드 (예: `KRW`, `BTC`, `USDT`) |
| `tick_size` | string | 해당 페어에 적용되는 호가 단위 |
| `supported_levels` | array | 호가 모아보기 단위 목록. 0: 기본 호가단위 (원화마켓에서만 다수 지원, BTC/USDT는 0만 존재) |

## 응답 예시

```json
[
  {
    "market": "KRW-BTC",
    "quote_currency": "KRW",
    "tick_size": 1000,
    "supported_levels": [0, 10000, 100000, 1000000, 10000000, 100000000]
  },
  {
    "market": "KRW-ETH",
    "quote_currency": "KRW",
    "tick_size": 1000,
    "supported_levels": [0, 10000, 100000, 1000000]
  }
]
```

## 변경 이력

| 일자 | 변경 사항 |
|------|----------|
| 2025-07-31 | 호가 정책 조회 기능 신규 지원 |

## Deprecated

`list-orderbook-levels.md` (호가 모아보기 단위 조회)는 `list-orderbook-instruments.md`로 대체됨.
