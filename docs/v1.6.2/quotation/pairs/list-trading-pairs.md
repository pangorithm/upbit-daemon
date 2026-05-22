# 페어 목록 조회

업비트에서 지원하는 모든 페어 목록을 조회합니다.

**Endpoint**: `GET https://api.upbit.com/v1/market/all`

**Rate Limit**: 초당 최대 10회 (IP 단위, 캔들 그룹 공유)

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `is_details` | boolean | 선택 | 상세 정보 포함 조회. true 시 유의종목/주의종목 정보 포함. 기본값 false |

## 응답 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `market` | string | 페어 코드 (예: `KRW-BTC`) |
| `korean_name` | string | 한글명 |
| `english_name` | string | 영문명 |
| `market_event.warning` | boolean | 유의 종목 여부 |
| `market_event.caution.PRICE_FLUCTUATIONS` | boolean | 가격 급등락 경보 |
| `market_event.caution.TRADING_VOLUME_SOARING` | boolean | 거래량 급증 경보 |
| `market_event.caution.DEPOSIT_AMOUNT_SOARING` | boolean | 입금량 급증 경보 |
| `market_event.caution.GLOBAL_PRICE_DIFFERENCES` | boolean | 국내외 가격 차이 경보 |
| `market_event.caution.CONCENTRATION_OF_SMALL_ACCOUNTS` | boolean | 소수 계정 집중 거래 경보 |

## 응답 예시

```json
[
  {
    "market": "KRW-BTC",
    "korean_name": "비트코인",
    "english_name": "Bitcoin",
    "market_event": {
      "warning": false,
      "caution": {
        "PRICE_FLUCTUATIONS": false,
        "TRADING_VOLUME_SOARING": false,
        "DEPOSIT_AMOUNT_SOARING": false,
        "GLOBAL_PRICE_DIFFERENCES": false,
        "CONCENTRATION_OF_SMALL_ACCOUNTS": false
      }
    }
  }
]
```

## 변경 이력

| 일자 | 변경 사항 |
|------|----------|
| 2024-11-20 | market_event 필드 신규 지원, market_warning 필드 필수 여부 변경 |
| 2024-02-22 | 페어별 시장경보 조회 지원 |
