# 개별 출금 조회

최신 단일 출금 정보를 조회합니다. UUID 또는 TXID로 특정 출금 조회 가능.

**Endpoint**: `GET https://api.upbit.com/v1/withdraw`

**Rate Limit**: 초당 최대 30회 (계정 단위, exchange default 그룹)

**API Key Permission**: 출금조회 권한 필요

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `uuid` | string | 선택 | 출금 UUID |
| `txid` | string | 선택 | 트랜잭션 ID |
| `currency` | string | 선택 | 통화 코드. 미입력 시 최신 출금 내역 반환 |

> `uuid`와 `txid`를 모두 입력하지 않으면 최신 출금 정보가 반환됩니다.

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `type` | string | `withdraw` |
| `uuid` | string | 출금 UUID |
| `currency` | string | 통화 코드 |
| `net_type` | string/null | 네트워크 유형 (KRW 출금 시 null) |
| `txid` | string/null | 트랜잭션 ID |
| `state` | string | `WAITING` / `PROCESSING` / `DONE` / `FAILED` / `CANCELLED` / `REJECTED` |
| `created_at` | string | 생성 시간 |
| `done_at` | string/null | 완료 시간 |
| `amount` | string | 출금 수량 |
| `fee` | string | 수수료 |
| `transaction_type` | string | `default` / `internal` |
| `is_cancelable` | boolean | 취소 가능 여부 |

## 변경 이력

| 일자 | 변경 사항 |
|------|----------|
| 2023-05-22 | 네트워크 타입(net_type) 필드 추가 |
| 2020-05-29 | `transaction_type` 필드 추가 |
