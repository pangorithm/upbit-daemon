# 분(Minute) 캔들 조회

분 단위 캔들 목록을 조회합니다.

**Endpoint**: `GET https://api.upbit.com/v1/candles/minutes/{unit}`

**Rate Limit**: 초당 최대 10회 (IP 단위, 캔들 그룹 공유)

## 단위 (Unit)

Path 파라미터 `unit`으로 캔들 너비 지정 가능: **1, 3, 5, 10, 15, 30, 60, 240분**

## 주의사항

- 캔들은 해당 시간대에 체결이 발생한 경우에만 생성됨

## 파라미터

| 파라미터 | 위치 | 형식 | 필수 | 설명 |
|---------|------|------|------|------|
| `unit` | Path | integer | 필수 | 캔들 단위 (1, 3, 5, 10, 15, 30, 60, 240) |
| `market` | Query | string | 필수 | 페어 코드 |
| `to` | Query | string | 선택 | 종료 시각 (ISO 8601). 미지정 시 요청 시각 기준 |
| `count` | Query | integer | 선택 | 캔들 개수. 최대 200, 기본값 1 |

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `market` | string | 페어 코드 |
| `candle_date_time_utc` | string | 시작 시각 (UTC) 포맷: `yyyy-MM-dd'T'HH:mm:ss` |
| `candle_date_time_kst` | string | 시작 시각 (KST) 포맷: `yyyy-MM-dd'T'HH:mm:ss` |
| `opening_price` | number | 시가 |
| `high_price` | number | 고가 |
| `low_price` | number | 저가 |
| `trade_price` | number | 종가 |
| `timestamp` | integer | 타임스탬프 (ms) |
| `candle_acc_trade_price` | number | 누적 거래 금액 |
| `candle_acc_trade_volume` | number | 누적 거래량 |
| `unit` | integer | 캔들 집계 시간 단위 (분) |
