# 월(Month) 캔들 조회

월 단위 캔들 목록을 조회합니다.

**Endpoint**: `GET https://api.upbit.com/v1/candles/months`

**Rate Limit**: 초당 최대 10회 (IP 단위, 캔들 그룹 공유)

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `market` | string | 필수 | 페어 코드 |
| `to` | string | 선택 | 종료 시각 (ISO 8601). 미지정 시 요청 시각 기준 |
| `count` | integer | 선택 | 캔들 개수. 최대 200, 기본값 1 |

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `market` | string | 페어 코드 |
| `candle_date_time_utc` | string | 시작 시각 (UTC) |
| `candle_date_time_kst` | string | 시작 시각 (KST) |
| `opening_price` | number | 시가 |
| `high_price` | number | 고가 |
| `low_price` | number | 저가 |
| `trade_price` | number | 종가 |
| `timestamp` | integer | 타임스탬프 (ms) |
| `candle_acc_trade_price` | number | 누적 거래 금액 |
| `candle_acc_trade_volume` | number | 누적 거래량 |
| `first_day_of_period` | string | 캔들 집계 시작일자 (`yyyy-MM-dd`) |
