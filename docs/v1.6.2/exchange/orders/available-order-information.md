# 페어별 주문 가능 정보 조회

지정한 페어의 주문 가능 정보를 조회합니다.

**Endpoint**: `GET https://api.upbit.com/v1/orders/chance`

**Rate Limit**: 초당 최대 30회 (계정 단위, exchange default 그룹)

**API Key Permission**: 주문조회 권한 필요

## 주요 응답 항목

| 항목 | 관련 필드 |
|-----|---------|
| 적용 수수료율 | `bid_fee`, `ask_fee`, `maker_bid_fee`, `maker_ask_fee` |
| 지원 주문 방향 및 유형 | `market.order_sides`, `market.bid_types`, `market.ask_types` |
| 기준/호가 자산의 최소/최대 주문 금액 | `market.bid`, `market.ask`, `market.max_total` |
| 기준/호가 자산의 계정 잔고 | `bid_account`, `ask_account` |

> **Deprecated**: `market.order_types` 필드 지원 종료 예정. `ask_types`, `bid_types` 필드 사용 권장.

## 파라미터

| 파라미터 | 형식 | 필수 | 설명 |
|---------|------|------|------|
| `market` | string | 필수 | 페어 코드 |

## 응답 필드

### 최상위 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `bid_fee` | string | 매수 수수료율 |
| `ask_fee` | string | 매도 수수료율 |
| `maker_bid_fee` | string | 매수 maker 수수료비율 |
| `maker_ask_fee` | string | 매도 maker 수수료비율 |
| `market` | object | 페어 정보 |
| `bid_account` | object | 호가 자산 계좌 정보 |
| `ask_account` | object | 기준 자산 계좌 정보 |

### market 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `id` | string | 페어 코드 |
| `name` | string | 페어 명 (`BTC/KRW`) |
| `order_types` | array[] | 지원하는 주문 유형 (deprecated) |
| `order_sides` | array[] | 지원하는 주문 방향 (`ask`, `bid`) |
| `bid_types` | array[] | 지원하는 매수 주문 유형 (`best_fok`, `best_ioc`, `limit`, `limit_fok`, `limit_ioc`, `price`) |
| `ask_types` | array[] | 지원하는 매도 주문 유형 |
| `bid` | object | 매수 제약 조건 (`currency`, `min_total`) |
| `ask` | object | 매도 제약 조건 (`currency`, `min_total`) |
| `max_total` | string | 최대 주문 가능 금액 |
| `state` | string | 페어 운영 상태 (`active`) |

### bid_account / ask_account 필드

| 필드 | 형식 | 설명 |
|-----|------|------|
| `currency` | string | 통화 코드 |
| `balance` | string | 주문 가능 수량/금액 |
| `locked` | string | 잠겨 있는 잔액 |
| `avg_buy_price` | string | 매수 평균가 |
| `avg_buy_price_modified` | boolean | 매수 평균가 수정 여부 |
| `unit_currency` | string | 평균가 기준 통화 |

## 변경 이력

| 일자 | 변경 사항 |
|------|----------|
| 2024-04-22 | 최유리지정가 주문 유형, 주문 옵션 추가 |
| 2022-10-14 | `order_types` deprecated, `ask_types`, `bid_types` 추가 |
