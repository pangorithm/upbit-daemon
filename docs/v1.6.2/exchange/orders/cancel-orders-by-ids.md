# id로 주문 목록 취소 접수

UUID 또는 Identifier 목록으로 취소 대상 주문을 지정 취소합니다. 한 번에 최대 20개.

**Endpoint**: `DELETE https://api.upbit.com/v1/orders/uuids`

**Rate Limit**: 초당 최대 30회 (계정 단위, exchange default 그룹)

**API Key Permission**: 주문하기 권한 필요

## 주의사항

- `uuids[]` 또는 `identifiers[]` 중 하나를 반드시 포함
- 두 파라미터를 동시에 사용할 수 없음
- 쿼리 파라미터 형식만 지원 (Request Body 불가)
- 취소 요청이 거절될 수 있는 경우: 전량 체결 완료, 이미 취소 완료, 페어 서비스 일시 정지

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `uuids[]` | string[] | 조건부 | 취소할 UUID 목록 (최대 20개). 예: `uuids[]=uuid1&uuids[]=uuid2` |
| `identifiers[]` | string[] | 조건부 | 취소할 identifier 목록 (최대 20개). 예: `identifiers[]=id1&identifiers[]=id2` |

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `success` | object | 성공적으로 취소된 주문 정보 |
| `success.count` | number | 성공 개수 |
| `success.orders` | array | 성공 주문 목록 (`uuid`, `market`, `identifier`) |
| `failed` | object | 취소 실패한 주문 정보 |
| `failed.count` | number | 실패 개수 |
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
| 2024-12-11 | 지정 주문 목록 취소 기능 신규 지원 |
