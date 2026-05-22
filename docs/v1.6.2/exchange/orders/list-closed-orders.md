# 종료 주문 목록 조회

종료 주문(Closed Order: 전량 체결 + 취소 주문) 목록을 조회합니다.

**Endpoint**: `GET https://api.upbit.com/v1/orders/closed`

**Rate Limit**: 초당 최대 30회 (계정 단위, exchange default 그룹)

**API Key Permission**: 주문조회 권한 필요

> `state`와 `states[]`는 동시에 사용할 수 없습니다. 조회 기간 최대 7일.

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `market` | string | 선택 | 페어 코드 |
| `state` | string | 선택 | `done`(체결 완료) / `cancel`(취소). 미지정 시 모든 상태 반환 |
| `states[]` | string[] | 선택 | 여러 상태 필터. 기본값: `["done", "cancel"]` |
| `start_time` | string | 선택 | 조회 시작 시각 (ISO 8601 또는 ms 타임스탬프) |
| `end_time` | string | 선택 | 조회 종료 시각 (ISO 8601 또는 ms 타임스탬프) |
| `limit` | integer | 선택 | 요청 개수. 최대 1000, 기본값 100 |
| `order_by` | string | 선택 | 정렬 (`desc`: 최신순, `asc`: 오래된순). 기본값 `desc` |

> `start_time`만 입력 시 해당 시각 기준 7일, `end_time`만 입력 시 해당 시각 기준 이전 7일, 둘 다 미입력 시 요청 시각 기준 이전 7일.

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `market` | string | 페어 코드 |
| `uuid` | string | 주문 UUID |
| `side` | string | `ask` / `bid` |
| `ord_type` | string | 주문 유형 |
| `price` | string | 단가/총액 |
| `state` | string | `done` / `cancel` |
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
| 2024-12-18 | `start_time`, `end_time` timestamp 형식 추가 |
| 2024-12-04 | `identifier` 필드 신규 지원 |
| 2024-10-02 | 조회 가능 범위 7일로 확대 |
| 2024-06-27 | 종료 주문 목록 조회 신규 지원 |
