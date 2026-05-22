# 출금 가능 정보 조회

지정한 통화에 대한 출금 가능 정보를 조회합니다. 출금 정책과 사용자 잔고를 확인할 수 있습니다.

**Endpoint**: `GET https://api.upbit.com/v1/withdraws/chance`

**Rate Limit**: 초당 최대 30회 (계정 단위, exchange default 그룹)

**API Key Permission**: 출금조회 권한 필요

## 주요 응답 항목

| 항목 | 관련 필드 |
|-----|---------|
| 통화 정보 (수수료, 지갑 상태) | `currency.withdraw_fee`, `currency.wallet_state`, `currency.wallet_support` |
| 통화 잔고 | `account.balance`, `account.locked`, `account.avg_buy_price` |
| 출금 한도 (1회/일일/잔여) | `withdraw_limit.onetime`, `withdraw_limit.daily`, `withdraw_limit.remaining_daily` |
| 계정 정보 (수수료 레벨, 인증) | `member_level.fee_level`, `member_level.bank_account_verified` 등 |

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `currency` | string | 필수 | 통화 코드 |
| `net_type` | string | 디지털 자산 필수 | 네트워크 유형 |

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `member_level` | object | 회원 등급 정보 (`security_level`, `fee_level`, `email_verified`, `identity_auth_verified`, `bank_account_verified`, `two_factor_auth_verified`, `locked`, `wallet_locked`) |
| `currency` | object | 통화 정보 (`code`, `withdraw_fee`, `is_coin`, `wallet_state`, `wallet_support`) |
| `account` | object | 자산 잔고 (`currency`, `balance`, `locked`, `avg_buy_price`, `avg_buy_price_modified`, `unit_currency`) |
| `withdraw_limit` | object | 출금 제한 (`currency`, `onetime`, `daily`, `remaining_daily`, `remaining_daily_fiat`, `fiat_currency`, `minimum`, `fixed`, `withdraw_delayed_fiat`, `can_withdraw`) |

## 변경 이력

| 일자 | 변경 사항 |
|------|----------|
| 2023-05-22 | 네트워크 타입(net_type) 필드 추가 |
