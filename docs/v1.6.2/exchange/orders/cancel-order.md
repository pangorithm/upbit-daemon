# 개별 주문 취소 접수

UUID 또는 Identifier로 주문을 취소합니다.

**Endpoint**: `DELETE https://api.upbit.com/v1/order`

**Rate Limit**: 초당 최대 30회 (계정 단위, exchange default 그룹)

**API Key Permission**: 주문하기 권한 필요

## 주의사항

- `uuid` 또는 `identifier` 중 하나를 반드시 포함해야 함
- 둘 다 지정 시 `uuid` 기준으로 취소

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `uuid` | string | 조건부 | 취소하고자 하는 주문의 UUID |
| `identifier` | string | 조건부 | 취소하고자 하는 주문의 클라이언트 지정 식별자 |

## 응답 필드

주문 생성 응답과 동일한 필드 구조 (state: `wait` 또는 `watch`)

## 응답 예시

```json
{
  "uuid": "cdd92199-2897-4e14-9448-f923320408ad",
  "side": "bid",
  "ord_type": "limit",
  "price": "140000000",
  "state": "wait",
  "market": "KRW-BTC",
  "created_at": "2025-07-04T15:00:00+09:00",
  "volume": "1.0",
  "remaining_volume": "1.0",
  "executed_volume": "0.0",
  "reserved_fee": "70000.0",
  "remaining_fee": "70000.0",
  "paid_fee": "0.0",
  "locked": "140070000.0",
  "trades_count": 0
}
```

## 변경 이력

| 일자 | 변경 사항 |
|------|----------|
| 2025-07-02 | SMP 필드 추가 (`smp_type`, `prevented_volume`, `prevented_locked`) |
| 2024-12-04 | `identifier` 필드 신규 지원 |
