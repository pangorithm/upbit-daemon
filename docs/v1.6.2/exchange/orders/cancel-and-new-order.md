# 취소 후 재주문

한 번의 요청으로 기존 주문을 취소하고 신규 주문을 생성합니다.

**Endpoint**: `POST https://api.upbit.com/v1/orders/cancel_and_new`

**Rate Limit**: 초당 최대 8회 (계정 단위, order 그룹)

**API Key Permission**: 주문하기 권한 필요

## 제약사항

- 신규 주문은 기존 주문과 **동일한 페어, 동일한 주문 방향**에서만 생성 가능
- 기존 주문 identifier는 재사용 불가
- 수량(`new_volume`), 금액(`new_price`) 등은 변경 가능
- 부분 체결 주문 시 `new_volume`에 `"remain_only"` 설정하면 잔량으로 신규 주문 가능

## 신규 주문 유형별 필수 파라미터

| 주문 유형 | 필수 필드 |
|---------|---------|
| 지정가 (limit) | `new_volume`, `new_price` |
| 시장가 매수 (price) | `new_price` |
| 시장가 매도 (market) | `new_volume` |
| 최유리 지정가 매수 (best, bid) | `new_price`, `new_time_in_force` |
| 최유리 지정가 매도 (best, ask) | `new_volume`, `new_time_in_force` |

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `prev_order_uuid` | string | 조건부 | 취소할 주문의 UUID |
| `prev_order_identifier` | string | 조건부 | 취소할 주문의 클라이언트 지정 ID |
| `new_ord_type` | string | 필수 | 신규 주문 유형 (`limit`, `price`, `market`, `best`) |
| `new_volume` | string | 주문유형별 | 신규 주문 수량 (`remain_only` 가능) |
| `new_price` | string | 주문유형별 | 신규 주문 단가 또는 총액 |
| `new_identifier` | string | 선택 | 신규 주문 클라이언트 ID (최대 32자) |
| `new_time_in_force` | string | 선택/필수 | `ioc`, `fok`, `post_only` (최유리 주문 시 필수) |
| `new_smp_type` | string | 선택 | SMP 모드 (`cancel_maker`, `cancel_taker`, `reduce`) |

> `prev_order_uuid` 또는 `prev_order_identifier` 중 하나를 반드시 포함해야 함

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| (기존 주문 정보) | - | market, uuid, side, ord_type, price, state, created_at, volume, remaining_volume, executed_volume, reserved_fee, remaining_fee, paid_fee, locked, trades_count, time_in_force, smp_type, prevented_volume, prevented_locked |
| `new_order_uuid` | string | 신규 생성된 주문 UUID |
| `new_order_identifier` | string | 신규 생성된 주문 클라이언트 ID |

## 응답 예시

```json
{
  "uuid": "ad217e24-ed02-469c-9b30-c08dbbda6908",
  "side": "bid",
  "ord_type": "limit",
  "price": "100000000",
  "state": "wait",
  "market": "KRW-BTC",
  "created_at": "2025-07-04T15:00:00+09:00",
  "volume": "1",
  "remaining_volume": "1",
  "executed_volume": "0.0",
  "reserved_fee": "70000.0",
  "remaining_fee": "70000.0",
  "paid_fee": "0.0",
  "locked": "100070000.0",
  "prevented_volume": "0",
  "prevented_locked": "0",
  "trades_count": 0,
  "new_order_uuid": "4b07aa31-4747-485c-8bce-ac5495e4a639"
}
```

## 변경 이력

| 일자 | 변경 사항 |
|------|----------|
| 2025-07-07 | Post Only 주문 옵션 신규 지원 |
| 2025-07-02 | SMP 필드 추가 |
| 2025-02-05 | 취소 후 재주문 API 신규 지원 |
