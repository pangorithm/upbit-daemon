# 체결 대기 주문 목록 조회

계정의 체결 대기 주문 목록을 조회합니다.

**Endpoint**: `GET https://api.upbit.com/v1/orders/open`

**Rate Limit**: 초당 최대 30회 (계정 단위, exchange default 그룹)

**API Key Permission**: 주문조회 권한 필요

> `state`와 `states[]`는 동시에 사용할 수 없습니다.

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `market` | string | 선택 | 페어 코드 |
| `state` | string | 선택 | 주문 상태 (`wait`: 체결 대기, `watch`: 예약 주문 대기). 기본값 `wait` |
| `states[]` | string[] | 선택 | 여러 상태 필터. 예: `states[]=wait&states[]=watch` |
| `page` | integer | 선택 | 페이지 번호. 기본값 1 |
| `limit` | integer | 선택 | 요청 개수. 최대 100, 기본값 100 |
| `order_by` | string | 선택 | 정렬 (`desc`: 최신순, `asc`: 오래된순). 기본값 `desc` |

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `market` | string | 페어 코드 |
| `uuid` | string | 주문 UUID |
| `side` | string | `ask` / `bid` |
| `ord_type` | string | 주문 유형 |
| `price` | string | 단가/총액 |
| `state` | string | `wait` / `watch` |
| `created_at` | string | 생성 시각 (KST) |
| `volume` | string | 요청 수량 |
| `remaining_volume` | string | 남은 양 |
| `executed_volume` | string | 체결된 양 |
| `executed_funds` | string | 체결된 금액 |
| `reserved_fee` | string | 수수료 예약 비용 |
| `remaining_fee` | string | 남은 수수료 |
| `paid_fee` | string | 사용된 수수료 |
| `locked` | string | 사용 중인 비용 |
| `time_in_force` | string | 체결 옵션 |
| `identifier` | string | 클라이언트 ID |
| `smp_type` | string | SMP 모드 |
| `prevented_volume` | string | SMP로 취소된 수량 |
| `prevented_locked` | string | SMP로 해제된 자산 |
| `trades_count` | integer | 체결 건수 |

## 변경 이력

| 일자 | 변경 사항 |
|------|----------|
| 2025-07-02 | SMP 필드 추가 |
| 2024-12-04 | `identifier` 필드 신규 지원 |
| 2024-06-27 | 체결 대기 주문 목록 조회 신규 지원 |
