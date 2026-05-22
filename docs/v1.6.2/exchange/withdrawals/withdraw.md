# 디지털 자산 출금 요청

디지털 자산 출금을 요청합니다.

**Endpoint**: `POST https://api.upbit.com/v1/withdraws/coin`

**Rate Limit**: 초당 최대 30회 (계정 단위, exchange default 그룹)

**API Key Permission**: 출금하기 권한 필요

## 사전 준비

- 출금 대상 주소는 반드시 **출금 허용 주소로 사전 등록**해야 함 (업비트 홈페이지를 통해 등록, API 등록 불가)
- 네트워크 타입(`net_type`)은 필수 파라미터. [출금 허용 주소 목록 조회](./list-withdrawal-addresses.md) API로 확인

## 출금 유형

| 유형 | 값 | 설명 |
|-----|-----|------|
| 일반 출금 | `default` | 블록체인 트랜잭션. 확정 시간 + 수수료 발생 |
| 바로 출금 | `internal` | 블록체인 미사용. 업비트 계정 간 송금. 약 1분, 수수료 없음. **업비트 회원 지갑 주소로만 출금 가능** |

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `currency` | string | 필수 | 출금 통화 코드 (예: `BTC`) |
| `net_type` | string | 필수 | 네트워크 유형. 출금 주소 목록 조회 API의 `net_type` 값 사용 |
| `amount` | string | 필수 | 출금 수량 |
| `address` | string | 필수 | 출금 수신 주소 (등록된 주소만 사용 가능) |
| `secondary_address` | string | 선택 | 2차 출금 주소 (Destination Tag, Memo 등). 일부 자산 필수 |
| `transaction_type` | string | 선택 | `default` / `internal`. 기본값 `default` |

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `type` | string | `withdraw` |
| `uuid` | string | 출금 UUID |
| `currency` | string | 통화 코드 |
| `net_type` | string | 네트워크 유형 |
| `txid` | string/null | 트랜잭션 ID |
| `state` | string | `WAITING` / `PROCESSING` / `DONE` / `FAILED` / `CANCELLED` / `REJECTED` |
| `created_at` | string | 생성 시간 |
| `done_at` | string/null | 완료 시간 (완료 전이면 null) |
| `amount` | string | 출금 수량 |
| `fee` | string | 수수료 |
| `transaction_type` | string | 출금 유형 |
| `is_cancelable` | boolean | 취소 가능 여부 |

## 변경 이력

| 일자 | 변경 사항 |
|------|----------|
| 2023-05-22 | 네트워크 타입(net_type) 필드 추가 |
| 2020-05-29 | `transaction_type` 필드 추가 |
| 2019-04-23 | 바로 출금 기능 지원 |
