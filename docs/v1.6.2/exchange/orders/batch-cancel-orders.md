# 주문 일괄 취소 접수

조건을 지정하여 해당 조건을 만족하는 최대 300개의 체결 대기 주문을 일괄 취소합니다.

**Endpoint**: `DELETE https://api.upbit.com/v1/orders/open`

**Rate Limit**: 2초당 최대 1회

**API Key Permission**: 주문하기 권한 필요

> **주의**: 쿼리 파라미터 형식만 지원합니다 (Request Body 불가). 페어(`pairs`)와 마켓(`quote_currencies`) 조건은 동시에 사용할 수 없습니다.

## 일괄 취소 가능 여부

- **체결 대기(WAIT)** 상태의 주문만 취소 가능
- **예약 주문(WATCH)** 상태는 개별 취소 또는 id로 주문 목록 취소 API 사용 필요
- 취소 처리 중 체결이 발생할 수 있으므로, 요청 시점의 잔량과 실제 취소된 잔량은 다를 수 있음

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `cancel_side` | string | 선택 | 취소 방향 (`all` / `bid` / `ask`). 기본값 `all` |
| `pairs` | string | 선택 | 페어 목록 (최대 20개). 예: `KRW-BTC,KRW-ETH` |
| `quote_currencies` | string | 선택 | 마켓(호가 자산) 목록. 예: `KRW,BTC` |
| `exclude_pairs` | string | 선택 | 제외할 페어 목록 (최대 20개) |
| `count` | integer | 선택 | 취소할 최대 개수. 최대 300, 기본값 20 |
| `order_by` | string | 선택 | 정렬 방식 (`desc`: 최신순, `asc`: 오래된순). 기본값 `desc` |

> **우선순위**: `exclude_pairs` → `pairs` / `quote_currencies` 순으로 적용

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `success.count` | number | 성공적으로 취소된 주문 수 |
| `success.orders` | array | 성공 주문 목록 (`uuid`, `market`, `identifier`) |
| `failed.count` | number | 취소 실패한 주문 수 |
| `failed.orders` | array | 실패 주문 목록 (`uuid`, `market`, `identifier`) |

## 응답 예시

```json
{
  "success": {
    "count": 2,
    "orders": [
      {"uuid": "bbbb8e07-1689-4769-af3e-a117016623f8", "market": "KRW-ETH"},
      {"uuid": "4312ba49-5f1a-4a01-9f3b-2d2bce17267e", "market": "KRW-ETH"}
    ]
  },
  "failed": {
    "count": 1,
    "orders": [
      {"uuid": "bdb49a54-de36-4eb4-a963-9c8d4337a9da", "market": "BTC-XRP"}
    ]
  }
}
```

## 변경 이력

| 일자 | 변경 사항 |
|------|----------|
| 2024-12-11 | 주문 일괄 취소 기능 신규 지원 |
