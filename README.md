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
│   │   ├── auth.rs          # JWT 생성
│   │   ├── quotation/
│   │   │   └── candle.rs    # 시세 조회 - 분 캔들
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

candle_unit: 10
batch_size: 200
api_calls_per_second: 5
partition_retain_days: 30
partition_retain_months: 6
partition_create: 3
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
| tickers | 일 | SQL (현재월) + 프로그램 |
| trades | 일 | SQL (현재월) + 프로그램 |
| candles_seconds | 일 | SQL (현재월) + 프로그램 |
| candles_minutes | 월 | SQL (현재월) + 프로그램 |
| candles_days | 월 | SQL (현재월) + 프로그램 |
| orderbooks | - | 단일 테이블 |

**프로그램 시작 시:**
1. 기존 파티션 조회
2. 월별 테이블 (candles_minutes, candles_days): 마지막 파티션 ~ 현재 월 사이 빈 칸 생성 (gap-filling)
3. 일별 테이블 (tickers, trades, candles_seconds): 마지막 파티션 ~ 현재 일 사이 빈 칸 생성 (gap-filling)
4. 다음 3개월분 월 파티션 생성

**Cron (일일):**
1. 다음 3개월분 월 파티션 생성 (이미 있으면 skip)

## 실행 시나리오

### 1. 데몬 프로그램 실행
- `cargo run` 또는 시스템 서비스로 실행
- `.env`에서 API 키, DB 연결 정보 로드
- `config.yaml`에서 수집 설정 (페어 목록, 간격 등) 로드

### 2. 디비 연결
- `sqlx`를 통해 PostgreSQL 17에 연결 풀 (pool) 생성
- 연결 실패 시 재시도 후 종료

### 3. 테이블 초기화
- 프로그램 시작 시 핵심 테이블 (markets, tickers, trades, candles_*, orderbooks) 이 존재하는지 확인
- 초기화 안 되어 있으면 `migrations/`의 스키마 기반으로 자동 생성

### 4. 파티션 gap-filling
- 각 파티션 테이블에서 마지막 생성된 파티션 확인
- 마지막 파티션부터 현재 시점까지 누락된 파티션 테이블 자동 생성
  - `candles_seconds`: 일 단위 파티션 (누락된 일자 생성)
  - `candles_minutes`, `candles_days`, `tickers`, `trades`: 월 단위 파티션 (누락된 월 생성)

### 5. 미래 파티션 생성 (Cron)
- 프로그램 시작 시 다음 3개월분 파티션 자동 생성
- Cron으로 일일 실행하여 미래 파티션 계속 생성 (이미 있으면 skip)

### 6. 과거 파티션 삭제 (Cron)
- `tickers` (일 단위): **1개월 이상** 경과된 파티션 삭제
- `trades` (일 단위): **1개월 이상** 경과된 파티션 삭제
- `candles_seconds` (일 단위): **1개월 이상** 경과된 파티션 삭제
- `candles_minutes` (월 단위): **6개월 이상** 경과된 파티션 삭제
- 저장 공간 관리 및 쿼리 성능 유지

### 7. 페어 목록 조회
- REST API (`/v1/markets`) 로 업비트 전체 페어 목록 조회
- **Cron으로 1일 1회** 실행 (실시간 실행도 가능)
- `markets` 테이블에 `UPSERT` (동시성 처리: `ON CONFLICT DO UPDATE`)
  - 신규 페어 추가 / 기존 페어 정보 업데이트

### 8. 1분봉 캔들 구독 관리
- 프로그램 시작 시 `markets` 테이블에서 **구독 중인 페어** 목록 조회
- 구독 중이지 않은 페어 중 1분봉 캔들이 없는 경우 자동 구독 추가
- **Cron으로 1일 1회** 실행하여 신규 페어 발견 시 자동 구독
- WebSocket 스트림에 동적 구독 메시지 전송

### 9. 10분봉 캔들 gap-filling (구독 시작 시)
- 10분봉 구독 시작 전, 해당 페어의 마지막 캔들 시간과 현재 시간 비교
- 누락된 캔들 확인 시 REST API (`/v1/candles/minutes/10`) 로 조회
- 마지막 캔들이 없으면 gap-filling 하지 않음 (새 구독)
- REST API 응답 데이터를 DB에 `UPSERT`

### 10. 수신 데이터 DB Upsert
- WebSocket으로 수신된 모든 데이터 (캔들, 호가, 체결 등)를 실시간으로 DB에 저장
- `INSERT ... ON CONFLICT DO UPDATE` 방식으로 중복 방지
- REST API로 수집된 데이터도 동일하게 upsert

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

## 완료된 단계

| 단계 | 내용 | 상태 |
|------|------|------|
| 1 | 프로젝트 초기화, Cargo.toml, .env | 완료 |
| 2 | 마이그레이션 (파티션 테이블) | 완료 |
| 3 | config.rs (YAML + .env 로드) | 완료 |
| 4 | db.rs (sqlx Pool, 연결 관리) | 완료 |
| 5 | api/rest.rs (REST API 호출) | 완료 |
| 6 | api/websocket.rs (WebSocket 구독) | 완료 |
| 7 | api/auth.rs (JWT 생성) | 완료 |
| 8 | api/candles_api.rs (캔들 조회 API) | 완료 |
| 9 | collector/candles.rs (캔들 gap-filling) | 완료 |
