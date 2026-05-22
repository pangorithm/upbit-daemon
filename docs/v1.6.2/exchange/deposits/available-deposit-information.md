# 디지털 자산 입금 가능 정보 조회

지정한 통화에 대한 입금 가능 정보를 조회합니다.

**Endpoint**: `GET https://api.upbit.com/v1/deposits/chance/coin`

**Rate Limit**: 초당 최대 30회 (계정 단위, exchange default 그룹)

**API Key Permission**: 입금조회 권한 필요

> **주의**: 실시간 상태 조회를 보장하지 않습니다. 수 분 정도 지연될 수 있으며, 거래 전략 용도가 아닌 참고용으로만 사용 권장. 실제 입금 전 업비트 공지사항 및 실시간 입출금 현황 페이지 확인 필수.

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `currency` | string | 필수 | 통화 코드 |
| `net_type` | string | 필수 | 네트워크 유형 |

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `currency` | string | 통화 코드 |
| `net_type` | string/null | 네트워크 유형 |
| `is_deposit_possible` | boolean | 입금 가능 여부 |
| `deposit_impossible_reason` | string | 입금 불가 이유 |
| `minimum_deposit_amount` | string | 최소 입금 수량 |
| `minimum_deposit_confirmations` | integer | 최소 입금 컨펌 수 |
| `decimal_precision` | integer | 소수점 자릿수 |

## 변경 이력

| 일자 | 변경 사항 |
|------|----------|
| 2024-11-14 | 입금 가능 정보 조회 기능 신규 추가 |
