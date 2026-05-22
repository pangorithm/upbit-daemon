# 원화 출금 요청

원화(KRW) 출금을 요청합니다.

**Endpoint**: `POST https://api.upbit.com/v1/withdraws/krw`

**Rate Limit**: 초당 최대 30회 (계정 단위, exchange default 그룹)

**API Key Permission**: 출금하기 권한 필요

## 2채널 인증

원화 출금 시 다음 중 하나의 2차 인증 완료 필요:

| 인증 수단 | 값 |
|---------|-----|
| 카카오 인증 | `kakao` |
| 네이버 인증 | `naver` |
| 하나 인증서 | `hana` |

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `amount` | string | 필수 | 출금 원화 금액 |
| `two_factor_type` | string | 필수 | 2차 인증 수단 (`kakao` / `naver` / `hana`) |

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `type` | string | `withdraw` |
| `uuid` | string | 출금 UUID |
| `currency` | string | `KRW` |
| `txid` | string/null | 트랜잭션 ID |
| `state` | string | `WAITING` / `PROCESSING` / `DONE` / `FAILED` / `CANCELLED` / `REJECTED` |
| `created_at` | string | 생성 시간 |
| `done_at` | string/null | 완료 시간 |
| `amount` | string | 출금 금액 |
| `fee` | string | 수수료 |
| `transaction_type` | string | 출금 유형 |
| `is_cancelable` | boolean | 취소 가능 여부 |

## 변경 이력

| 일자 | 변경 사항 |
|------|----------|
| 2022-09-05 | 네이버 인증 수단 추가 |
| 2021-02-08 | 원화 출금 요청 기능 재개 |
| 2020-05-29 | `transaction_type` 필드 추가 |
