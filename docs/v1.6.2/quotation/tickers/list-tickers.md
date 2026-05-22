# 페어 단위 현재가 조회

지정한 페어의 현재가를 조회합니다. 요청 시점 기준으로 해당 페어의 티커 스냅샷이 반환됩니다.

**Endpoint**: `GET https://api.upbit.com/v1/ticker`

**Rate Limit**: 초당 최대 10회 (IP 단위, ticker 그룹)

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `markets` | string | 필수 | 페어 목록. 여러 페어는 쉼표 구분 (예: `KRW-BTC,KRW-ETH`) |

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `market` | string | 페어 코드 |
| `trade_date` | string | 최근 체결 일자 (UTC) `yyyyMMdd` |
| `trade_time` | string | 최근 체결 시각 (UTC) `HHmmss` |
| `trade_date_kst` | string | 최근 체결 일자 (KST) |
| `trade_time_kst` | string | 최근 체결 시각 (KST) |
| `trade_timestamp` | integer | 체결 시각 타임스탬프 (ms) |
| `opening_price` | number | 시가 |
| `high_price` | number | 고가 |
| `low_price` | number | 저가 |
| `trade_price` | number | 현재 가격 (종가) |
| `prev_closing_price` | number | 전일 종가 (UTC 0시 기준) |
| `change` | string | 가격 변동 상태 (`EVEN` / `RISE` / `FALL`) |
| `change_price` | number | 전일 종가 대비 가격 변화 (절대값) |
| `change_rate` | number | 전일 종가 대비 가격 변화율 (절대값) |
| `signed_change_price` | number | 전일 종가 대비 가격 변화 (+/-) |
| `signed_change_rate` | number | 전일 종가 대비 가격 변화율 (+/-) |
| `trade_volume` | number | 최근 거래 수량 |
| `acc_trade_price` | number | 누적 거래 금액 (UTC 0시 기준) |
| `acc_trade_price_24h` | number | 24시간 누적 거래 금액 |
| `acc_trade_volume` | number | 누적 거래량 (UTC 0시 기준) |
| `acc_trade_volume_24h` | number | 24시간 누적 거래량 |
| `highest_52_week_price` | number | 52주 신고가 |
| `highest_52_week_date` | string | 52주 신고가 달성일 |
| `lowest_52_week_price` | number | 52주 신저가 |
| `lowest_52_week_date` | string | 52주 신저가 달성일 |
| `timestamp` | integer | 현재가 정보 타임스탬프 (ms) |

## 가격 변동 지표

- `change`, `change_price`, `change_rate`, `signed_change_price`, `signed_change_rate`는 **전일 종가(UTC 0시 기준)**를 기준으로 산출
