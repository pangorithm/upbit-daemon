# 출금 목록 조회

최신 출금 목록을 조회합니다. 조건을 설정해 해당 조건을 만족하는 출금 목록만 조회 가능.

**Endpoint**: `GET https://api.upbit.com/v1/withdraws`

**Rate Limit**: 초당 최대 30회 (계정 단위, exchange default 그룹)

**API Key Permission**: 출금조회 권한 필요

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `currency` | string | 선택 | 통화 코드. 미지정 시 모든 통화 |
| `state` | string | 선택 | 출금 상태 (`WAITING`, `PROCESSING`, `DONE`, `FAILED`, `CANCELLED`, `REJECTED`) |
| `uuids[]` | string[] | 선택 | UUID 목록 (최대 100개). `txids[]`와 동시 사용 불가 |
| `txids[]` | string[] | 선택 | TXID 목록 (최대 100개). `uuids[]`와 동시 사용 불가 |
| `limit` | integer | 선택 | 요청 개수. 최대 100, 기본값 100 |
| `page` | integer | 선택 | 페이지 번호. 기본값 1 |
| `order_by` | string | 선택 | 정렬 (`desc`: 최신순, `asc`: 오래된순). 기본값 `desc` |
| `from` | string | 선택 | 커서 (uuid 값). 해당 시각 이후 조회 |
| `to` | string | 선택 | 커서 (uuid 값). 해당 시각 이전 조회 |

## 응답 필드 (개별 출금 조회와 동일)

| 필드 | 형식 | 설명 |
|-----|------|------|
| `type` | string | `withdraw` |
| `uuid` | string | 출금 UUID |
| `currency` | string | 통화 코드 |
| `net_type` | string/null | 네트워크 유형 |
| `txid` | string/null | 트랜잭션 ID |
| `state` | string | 출금 처리 상태 |
| `created_at` | string | 생성 시간 |
| `done_at` | string/null | 완료 시간 |
| `amount` | string | 출금 수량 |
| `fee` | string | 수수료 |
| `transaction_type` | string | 출금 유형 |
| `is_cancelable` | boolean | 취소 가능 여부 |

## 변경 이력

| 일자 | 변경 사항 |
|------|----------|
| 2023-05-22 | 네트워크 타입(net_type) 필드 추가 |
| 2020-05-29 | `transaction_type` 필드 추가 |
| 2019-07-04 | `state`, `uuid`, `txid` 파라미터 신규 지원 |
