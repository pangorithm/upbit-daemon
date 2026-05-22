# 출금 허용 주소 목록 조회

계정에 등록된 출금 허용 주소 목록을 조회합니다.

**Endpoint**: `GET https://api.upbit.com/v1/withdraws/coin_addresses`

**Rate Limit**: 초당 최대 30회 (계정 단위, exchange default 그룹)

**API Key Permission**: 출금조회 권한 필요

## 사전 준비

출금 기능을 이용하기 위해 출금 주소 등록이 필수. 업비트 PC Web에서 [마이페이지 > Open API 관리 > 디지털 자산 출금주소 관리] 메뉴를 통해 등록.

## 네트워크 타입 (net_type) vs 네트워크 이름 (network_name)

| 필드 | 설명 |
|-----|------|
| `net_type` | 블록체인 네트워크 식별자 (예: BTC). 출금 시 필수 파라미터로 사용 |
| `network_name` | 사람이 인식할 수 있는 네트워크 전체 이름 (예: Bitcoin). UI 표시용 |

## 주소 종류별 응답 필드 차이

### 개인 지갑 주소
- `beneficiary_name`: 회원 이름
- `wallet_type`: 개인 지갑 이름 (예: "메타마스크")
- `exchange_name`, `beneficiary_type`: null

### 거래소 지갑
- `exchange_name`: 거래소 이름 (예: "바이낸스")
- `beneficiary_name`: 회원 이름
- `wallet_type`: null
- `beneficiary_type`: `individual`(개인) / `corporate`(법인)
- `beneficiary_company_name`: 법인명 (법인 소유 시)

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `currency` | string | 통화 코드 |
| `net_type` | string | 네트워크 유형 |
| `network_name` | string | 네트워크 이름 |
| `withdraw_address` | string | 출금 주소 |
| `secondary_address` | string/null | 2차 출금 주소 |
| `beneficiary_name` | string/null | 수취 지갑 소유주명 |
| `beneficiary_company_name` | string/null | 법인명 |
| `beneficiary_type` | string/null | `individual` / `corporate` |
| `exchange_name` | string/null | 거래소명 |
| `wallet_type` | string/null | 개인 지갑 종류 |

## 변경 이력

| 일자 | 변경 사항 |
|------|----------|
| 2025-07-07 | 수신 계정 관련 필드 추가 |
| 2023-11-22 | 네트워크 명(network_name) 필드 추가 |
| 2023-05-22 | 네트워크 타입(net_type) 필드 추가 |
