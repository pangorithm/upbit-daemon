# 호가 모아보기 단위 조회 (Deprecated)

종목별로 지원하는 모아보기 단위 목록을 조회합니다.

**Endpoint**: `GET https://api.upbit.com/v1/orderbook/supported_levels`

**Rate Limit**: 초당 최대 10회 (IP 단위, orderbook 그룹)

> **Deprecated**: `list-orderbook-instruments.md`(호가 정책 조회)로 대체됨.

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `markets` | string | 필수 | 페어 목록. 여러 페어는 쉼표 구분 |

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `market` | string | 페어 코드 |
| `supported_levels` | array | 호가 모아보기 단위 목록. 0: 기본 호가단위 |

## 응답 예시

```json
[
  {
    "market": "KRW-BTC",
    "supported_levels": [0, 10000, 100000, 1000000, 10000000, 100000000]
  },
  {
    "market": "KRW-ETH",
    "supported_levels": [0, 10000, 100000, 1000000]
  },
  {
    "market": "KRW-TRX",
    "supported_levels": [0, 1, 10, 100]
  }
]
```

> 호가 모아보기 기능은 현재 원화마켓만 지원합니다. 지원 대상 외 종목은 0만 반환.
