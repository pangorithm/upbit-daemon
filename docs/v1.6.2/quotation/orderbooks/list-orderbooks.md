# 호가 조회

지정한 종목들의 실시간 호가(Orderbook) 정보를 조회합니다.

**Endpoint**: `GET https://api.upbit.com/v1/orderbook`

**Rate Limit**: 초당 최대 10회 (IP 단위, orderbook 그룹)

## 호가 모아보기 (level)

원화마켓(KRW)에서만 지원하는 기능. 지정한 단위로 ask/bid price와 size를 모아(group) 조회.

- 숫자 형식 String. 0 또는 1 이상의 정수형, 또는 소수점 단위(double형)
- 미지정 시 기본값 0 (개별 호가)
- 미지원 단위 지정 시 빈 배열 반환 → 호출 전 [호가 정책 조회](./list-orderbook-instruments.md)로 확인 필요

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `markets` | string | 필수 | 페어 목록. 여러 페어는 쉼표 구분 |
| `level` | string | 선택 | 호가 모아보기 단위. 기본값 0 |
| `count` | integer | 선택 | 호가 쌍 개수. 최대 30, 기본값 30 |

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `market` | string | 페어 코드 |
| `timestamp` | integer | 요청 시각 타임스탬프 (ms) |
| `total_ask_size` | number | 전체 매도 잔량 합계 |
| `total_bid_size` | number | 전체 매수 잔량 합계 |
| `orderbook_units` | array | 호가 정보 (1호가부터 30호가까지) |
| `level` | number | 적용된 가격 단위 |

### orderbook_units 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `ask_price` | number | 매도 호가 |
| `bid_price` | number | 매수 호가 |
| `ask_size` | number | 매도 잔량 |
| `bid_size` | number | 매수 잔량 |

## 응답 예시

```json
[
  {
    "market": "KRW-BTC",
    "timestamp": 1751606867762,
    "total_ask_size": 10.37591054,
    "total_bid_size": 9.49577219,
    "orderbook_units": [
      {
        "ask_price": 148520000,
        "bid_price": 148490000,
        "ask_size": 0.0134662,
        "bid_size": 0.04296774
      }
    ],
    "level": 10000
  }
]
```

## 변경 이력

| 일자 | 변경 사항 |
|------|----------|
| 2025-07-02 | `count` 파라미터 신규 지원, 최대 30호가 지원 |
| 2024-01-22 | 호가 모아보기 기능 신규 지원 (원화 마켓) |
