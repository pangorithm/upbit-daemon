# 일(Day) 캔들 조회

일 단위 캔들 목록을 조회합니다.

**Endpoint**: `GET https://api.upbit.com/v1/candles/days`

**Rate Limit**: 초당 최대 10회 (IP 단위, 캔들 그룹 공유)

## 종가 환산 통화 (converting_price_unit)

원화 마켓 외 마켓(예: BTC 마켓)의 일 캔들에서 `converting_price_unit`을 `KRW`로 지정하면 `converted_trade_price` 필드에 원화 환산 종가 반환.

## 주의사항

- 캔들은 해당 시간대에 체결이 발생한 경우에만 생성됨

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `market` | string | 필수 | 페어 코드 |
| `to` | string | 선택 | 종료 시각 (ISO 8601). 미지정 시 요청 시각 기준 |
| `count` | integer | 선택 | 캔들 개수. 최대 200, 기본값 1 |
| `converting_price_unit` | string | 선택 | 종가 환산 통화. `KRW` 지정 시 `converted_trade_price` 필드 추가 반환 |

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
| `prev_closing_price` | number | 전일 종가 (UTC 0시 기준) |
| `change_price` | number | 전일 종가 대비 가격 변화 (`trade_price` - `prev_closing_price`) |
| `change_rate` | number | 전일 종가 대비 변화율 |
| `converted_trade_price` | number | 종가 환산 가격 (converting_price_unit 지정 시만 반환) |
