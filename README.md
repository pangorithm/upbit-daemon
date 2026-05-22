# upbit-daemon

업비트 API 정보 수집 데몬

## 기술 스택

| 영역 | 기술 |
|------|------|
| 언어 | Rust (2021 edition) |
| 비동기 런타임 | Tokio |
| REST API | reqwest |
| WebSocket | tokio-tungstenite |
| DB | PostgreSQL 17 (sqlx, 파티션) |
| 설정 | dotenvy (.env) + serde_yaml (config.yaml) |
| 로깅 | tracing + tracing-subscriber |
| 직렬화 | serde / serde_json |

## 아키텍처

```
┌─────────────────────────────────────────────────────┐
│                    upbit-daemon                      │
│                                                      │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐          │
│  │ REST     │  │ REST     │  │ REST     │          │
│  │ Collector│  │ Collector│  │ Collector│          │
│  │ tickers  │  │ candles  │  │ trades   │          │
│  │ (60초)   │  │ (1분)    │  │ (5분)    │          │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘          │
│       │             │             │                  │
│  ┌────┴─────────────┴─────────────┴─────┐          │
│  │           sqlx Pool                   │          │
│  └────────────────┬──────────────────────┘          │
│                   │                                  │
│  ┌────────────────┴──────────────────────┐          │
│  │     tokio-tungstenite (WebSocket)     │          │
│  │     orderbook subscription            │          │
│  └────────────────┬──────────────────────┘          │
└───────────────────┼─────────────────────────────────┘
                    │
          ┌─────────┴─────────┐
          │  PostgreSQL 17    │
          │  (파티션 테이블)   │
          └───────────────────┘
```

## 데이터 수집 간격

| API | 수집 간격 | 저장 테이블 |
|-----|----------|------------|
| tickers (현재가) | 60초 | tickers (월별 파티션) |
| candles (초) | 1초 | candles_seconds (일 단위 파티션) |
| candles (분) | 1분 | candles_minutes (월별 파티션) |
| candles (일) | 1분 | candles_days (월별 파티션) |
| trades (체결) | 5분 | trades (월별 파티션) |
| markets (페어) | 1시간 | markets |
| orderbooks (호가) | WebSocket 실시간 | orderbooks (인덱스) |

## 프로젝트 구조

```
upbit-daemon/
├── Cargo.toml
├── .env.example
├── .gitignore
├── migrations/
│   └── 001_initial.sql      # 파티션 테이블 스키마
├── src/
│   ├── main.rs              # 진입점, Tokio runtime
│   ├── config.rs            # 설정 로드 (.env + YAML)
│   ├── error.rs             # 에러 타입 정의
│   ├── db/
│   │   ├── mod.rs           # DB 연결, Pool 관리
│   │   └── models.rs        # sqlx 매크로 기반 모델
│   ├── api/
│   │   ├── mod.rs           # Upbit API 클라이언트
│   │   ├── rest.rs          # REST API 호출 (reqwest)
│   │   └── websocket.rs     # WebSocket 구독 (tokio-tungstenite)
│   └── collector/
│       ├── mod.rs           # 수집 코어
│       ├── tickers.rs       # 현재가 수집
│       ├── candles.rs       # 캔들 수집
│       ├── trades.rs        # 체결 수집
│       ├── markets.rs       # 페어 수집
│       └── orderbooks.rs    # 호가 WebSocket 수집
└── docs/                    # 업비트 API 문서
    └── v1.6.2/
```

## 설정

### .env (gitignore, 기밀 정보)

```env
UPBIT_ACCESS_KEY=
UPBIT_SECRET_KEY=
DATABASE_URL=postgresql://user:password@localhost:5432/upbit_daemon
```

### config.yaml (git 커밋, 수집 설정)

```yaml
markets:
  - KRW-BTC
  - KRW-ETH
  - KRW-XRP

collectors:
  tickers:
    interval_seconds: 60
  candles:
    interval_seconds: 60
  trades:
    interval_seconds: 300
  markets:
    interval_minutes: 60
```

## 데이터 모델 (API 응답 필드 그대로 사용)

### markets
| 컬럼 | 타입 | 설명 |
|------|------|------|
| market | VARCHAR(30) | 페어 코드 (PK) |
| korean_name | TEXT | 한글명 |
| english_name | TEXT | 영문명 |

### tickers (월별 파티션)
| 컬럼 | 타입 | 설명 |
|------|------|------|
| market | VARCHAR(30) | 페어 코드 |
| trade_date | VARCHAR(8) | 체결일자 (yyyyMMdd) |
| trade_time | VARCHAR(6) | 체결시각 (HHmmss) |
| trade_date_kst | VARCHAR(8) | KST 체결일자 |
| trade_time_kst | VARCHAR(6) | KST 체결시각 |
| trade_timestamp | BIGINT | 타임스탬프 (ms) |
| opening_price | DOUBLE PRECISION | 시가 |
| high_price | DOUBLE PRECISION | 고가 |
| low_price | DOUBLE PRECISION | 저가 |
| trade_price | DOUBLE PRECISION | 종가 |
| prev_closing_price | DOUBLE PRECISION | 전일 종가 |
| change | VARCHAR(10) | EVEN/RISE/FALL |
| change_price | DOUBLE PRECISION | 전일 종가 대비 변화 |
| change_rate | DOUBLE PRECISION | 변화율 |
| signed_change_price | DOUBLE PRECISION | 부호 있는 변화액 |
| signed_change_rate | DOUBLE PRECISION | 부호 있는 변화율 |
| trade_volume | DOUBLE PRECISION | 거래량 |
| acc_trade_price | DOUBLE PRECISION | 누적 거래금액 |
| acc_trade_price_24h | DOUBLE PRECISION | 24시간 누적 |
| acc_trade_volume | DOUBLE PRECISION | 누적 거래량 |
| acc_trade_volume_24h | DOUBLE PRECISION | 24시간 누적량 |
| highest_52_week_price | DOUBLE PRECISION | 52주 신고가 |
| lowest_52_week_price | DOUBLE PRECISION | 52주 신저가 |
| timestamp | BIGINT | 타임스탬프 |

### candles_seconds (일 단위 파티션)
| 컬럼 | 타입 | 설명 |
|------|------|------|
| market | VARCHAR(30) | 페어 코드 |
| candle_date_time_utc | VARCHAR(20) | 시작 시각 (UTC, ISO 8601) |
| candle_date_time_kst | VARCHAR(20) | 시작 시각 (KST) |
| opening_price | DOUBLE PRECISION | 시가 |
| high_price | DOUBLE PRECISION | 고가 |
| low_price | DOUBLE PRECISION | 저가 |
| trade_price | DOUBLE PRECISION | 종가 |
| candle_acc_trade_price | DOUBLE PRECISION | 누적 거래금액 |
| candle_acc_trade_volume | DOUBLE PRECISION | 누적 거래량 |

### candles_minutes (월별 파티션)
| 컬럼 | 타입 | 설명 |
|------|------|------|
| market | VARCHAR(30) | 페어 코드 |
| candle_date_time_utc | VARCHAR(20) | 시작 시각 (UTC) |
| candle_date_time_kst | VARCHAR(20) | 시작 시각 (KST) |
| opening_price | DOUBLE PRECISION | 시가 |
| high_price | DOUBLE PRECISION | 고가 |
| low_price | DOUBLE PRECISION | 저가 |
| trade_price | DOUBLE PRECISION | 종가 |
| candle_acc_trade_price | DOUBLE PRECISION | 누적 거래금액 |
| candle_acc_trade_volume | DOUBLE PRECISION | 누적 거래량 |
| unit | INTEGER | 캔들 단위 (1,3,5,10,15,30,60,240 분) |

### candles_days (월별 파티션)
| 컬럼 | 타입 | 설명 |
|------|------|------|
| market | VARCHAR(30) | 페어 코드 |
| candle_date_time_utc | VARCHAR(11) | 시작 시각 (UTC, yyyy-MM-dd) |
| candle_date_time_kst | VARCHAR(11) | 시작 시각 (KST) |
| opening_price | DOUBLE PRECISION | 시가 |
| high_price | DOUBLE PRECISION | 고가 |
| low_price | DOUBLE PRECISION | 저가 |
| trade_price | DOUBLE PRECISION | 종가 |
| candle_acc_trade_price | DOUBLE PRECISION | 누적 거래금액 |
| candle_acc_trade_volume | DOUBLE PRECISION | 누적 거래량 |
| prev_closing_price | DOUBLE PRECISION | 전일 종가 |
| change_price | DOUBLE PRECISION | 전일 종가 대비 변화 |
| change_rate | DOUBLE PRECISION | 변화율 |
| converted_trade_price | DOUBLE PRECISION | 원화 환산 종가 |

### trades (월별 파티션)
| 컬럼 | 타입 | 설명 |
|------|------|------|
| market | VARCHAR(30) | 페어 코드 |
| trade_date_utc | VARCHAR(10) | 체결 일자 |
| trade_time_utc | VARCHAR(12) | 체결 시각 |
| trade_price | DOUBLE PRECISION | 체결 가격 |
| trade_volume | DOUBLE PRECISION | 거래 수량 |
| sequential_id | BIGINT | 체결 ID (PK) |

### orderbooks
| 컬럼 | 타입 | 설명 |
|------|------|------|
| market | VARCHAR(30) | 페어 코드 |
| timestamp | BIGINT | 요청 시각 |
| total_ask_size | DOUBLE PRECISION | 전체 매도 잔량 |
| total_bid_size | DOUBLE PRECISION | 전체 매수 잔량 |
| orderbook_units | JSONB | 호가 단위 배열 |

## 파티션 구조

| 테이블 | 파티션 단위 | 생성 방식 |
|--------|------------|----------|
| tickers | 월 | SQL (현재월) + 프로그램 |
| trades | 월 | SQL (현재월) + 프로그램 |
| candles_seconds | 일 | SQL (현재월) + 프로그램 |
| candles_minutes | 월 | SQL (현재월) + 프로그램 |
| candles_days | 월 | SQL (현재월) + 프로그램 |
| orderbooks | - | 단일 테이블 |

**프로그램 시작 시:**
1. 기존 파티션 조회
2. 월별 테이블: 마지막 파티션 ~ 현재 월 사이 빈 칸 생성 (gap-filling)
3. 일별 테이블 (candles_seconds): 마지막 파티션 ~ 현재 일 사이 빈 칸 생성 (gap-filling)
4. 다음 3개월분 월 파티션 생성

**Cron (일일):**
1. 다음 3개월분 월 파티션 생성 (이미 있으면 skip)

## 실행

```bash
# 환경 변수 설정
cp .env.example .env
# .env 편집

# 마이그레이션 실행
DATABASE_URL=... cargo sqlx migrate run

# 실행
cargo run
```

## 개발 단계

| 단계 | 내용 | 상태 |
|------|------|------|
| 1 | 프로젝트 초기화, Cargo.toml, .env | 완료 |
| 2 | 마이그레이션 (파티션 테이블) | 완료 |
| 3 | config.rs (YAML + .env 로드) | 미실시 |
| 4 | db.rs (sqlx Pool, 연결 관리) | 미실시 |
| 5 | api/rest.rs (REST API 호출) | 미실시 |
| 6 | api/websocket.rs (WebSocket 구독) | 미실시 |
| 7 | collector/tickers.rs (현재가 수집) | 미실시 |
| 8 | collector/candles.rs (캔들 수집) | 미실시 |
| 9 | collector/trades.rs (체결 수집) | 미실시 |
| 10 | collector/orderbooks.rs (호가 WebSocket) | 미실시 |
| 11 | main.rs (조립) | 미실시 |
