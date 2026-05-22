# 페어 체결 이력 조회

지정한 페어의 최근 체결 목록을 조회합니다.

**Endpoint**: `GET https://api.upbit.com/v1/trades/ticks`

**Rate Limit**: 초당 최대 10회 (IP 단위, 체결 그룹)

## 조회 기간

최대 7일 이내의 체결 내역 조회 지원 (UTC 기준). `days_ago` 파라미터로 조회 대상 일자를 지정.

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `market` | string | 필수 | 페어 코드 |
| `to` | string | 선택 | 종료 시각 (HHmmss 또는 HH:mm:ss). 시간 역순으로 반환 |
| `count` | integer | 선택 | 체결 개수. 최대 500, 기본값 1 |
| `cursor` | string | 선택 | Pagination용 커서. 응답의 `sequential_id` 값을 입력하여 이어서 조회 |
| `days_ago` | integer | 선택 | 조회 대상 일자와 요청 시점의 일 단위 offset (1-7). 빈값 시 요청 일자 |

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `market` | string | 페어 코드 |
| `trade_date_utc` | string | 체결 일자 (UTC) `yyyy-MM-dd` |
| `trade_time_utc` | string | 체결 시각 (UTC) `HH:mm:ss` |
| `timestamp` | integer | 체결 시각 타임스탬프 (ms) |
| `trade_price` | number | 체결 가격 |
| `trade_volume` | number | 거래 수량 |
| `prev_closing_price` | number | 전일 종가 (UTC 0시 기준) |
| `change_price` | number | 전일 종가 대비 가격 변화 |
| `ask_bid` | string | 매수/매도 구분 (`ASK` / `BID`) |
| `sequential_id` | integer | 체결 유일 식별자 (체결 순서 보장 아님) |

## 변경 이력

| 일자 | 변경 사항 |
|------|----------|
| 2020-07-17 | 조회 기간 7일로 확대 지원 |
