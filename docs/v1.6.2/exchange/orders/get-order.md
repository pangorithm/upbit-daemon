# 개별 주문 조회

주문의 UUID 또는 Identifier로 단일 주문 정보를 조회합니다.

**Endpoint**: `GET https://api.upbit.com/v1/order`

**Rate Limit**: 초당 최대 30회 (계정 단위, exchange default 그룹)

**API Key Permission**: 주문조회 권한 필요

> `uuid` 또는 `identifier` 중 하나를 반드시 포함해야 함. 둘 다 지정 시 `uuid` 기준.

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `uuid` | string | 조건부 | 주문 UUID |
| `identifier` | string | 조건부 | 클라이언트 지정 주문 ID |

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `market` | string | 페어 코드 |
| `uuid` | string | 주문 UUID |
| `side` | string | `ask` / `bid` |
| `ord_type` | string | 주문 유형 |
| `price` | string | 단가/총액 |
| `state` | string | `wait` / `watch` / `done` / `cancel` |
| `created_at` | string | 생성 시각 (KST) |
| `volume` | string | 요청 수량 |
| `remaining_volume` | string | 남은 양 |
| `executed_volume` | string | 체결된 양 |
| `reserved_fee` | string | 수수료로 예약된 비용 |
| `remaining_fee` | string | 남은 수수료 |
| `paid_fee` | string | 사용된 수수료 |
| `locked` | string | 거래에 사용 중인 비용 |
| `time_in_force` | string | 체결 옵션 |
| `smp_type` | string | SMP 모드 |
| `prevented_volume` | string | SMP로 취소된 수량 |
| `prevented_locked` | string | SMP로 해제된 자산 |
| `identifier` | string | 클라이언트 지정 ID |
| `trades_count` | integer | 체결 건수 |
| `trades` | array[] | 체결 목록 |

### trades 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `market` | string | 페어 코드 |
| `uuid` | string | 체결 UUID |
| `price` | string | 체결 단가 |
| `volume` | string | 체결 수량 |
| `funds` | string | 체결 총액 |
| `trend` | string | `up`(매수 주문) / `down`(매도 주문) |
| `created_at` | string | 체결 시각 (KST) |
| `side` | string | `ask` / `bid` |

## 변경 이력

| 일자 | 변경 사항 |
|------|----------|
| 2025-07-02 | SMP 필드 추가 |
| 2024-12-04 | `identifier` 필드 신규 지원 |
| 2024-04-22 | 최유리지정가 주문, 주문 옵션 추가 |
