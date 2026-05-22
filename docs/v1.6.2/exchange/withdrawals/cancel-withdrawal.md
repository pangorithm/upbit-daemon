# 디지털 자산 출금 취소 요청

출금 UUID로 요청이 완료된 디지털 자산 출금 건의 취소를 요청합니다.

**Endpoint**: `DELETE https://api.upbit.com/v1/withdraws/coin`

**Rate Limit**: 초당 최대 30회 (계정 단위, exchange default 그룹)

**API Key Permission**: 출금하기 권한 필요

## 취소 가능 여부

- `is_cancelable` 필드로 취소 가능 여부 확인
- 통화의 출금 정책 및 네트워크 지연에 따라 실시간으로 변경

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `uuid` | string | 필수 | 취소할 출금 UUID |

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `type` | string | `withdraw` |
| `uuid` | string | 출금 UUID |
| `currency` | string | 통화 코드 |
| `net_type` | string | 네트워크 유형 |
| `txid` | string/null | 트랜잭션 ID |
| `state` | string | `CANCELLED` |
| `created_at` | string | 생성 시간 |
| `done_at` | string/null | 완료 시간 |
| `amount` | string | 출금 수량 |
| `fee` | string | 수수료 |
| `transaction_type` | string | 출금 유형 |
| `is_cancelable` | boolean | 취소 가능 여부 |

## 변경 이력

| 일자 | 변경 사항 |
|------|----------|
| 2025-05-19 | 디지털 자산 출금 취소 요청 신규 지원 |
