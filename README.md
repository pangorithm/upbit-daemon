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
| 설정 | clap (CLI) + serde_yaml (config.yaml) |
| 로깅 | tracing + tracing-subscriber |
| 직렬화 | serde / serde_json |

## 프로젝트 구조

```
upbit-daemon/
├── Cargo.toml
├── .env.example
├── .gitignore
├── config.yaml
├── migrations/
│   └── 001_initial.sql      # 파티션 테이블 스키마
├── src/
│   ├── main.rs              # 진입점, Tokio runtime
│   ├── config.rs            # CLI (clap) + YAML 설정 로드
│   ├── error.rs             # 에러 타입 정의
│   ├── db/
│   │   ├── mod.rs           # DB 연결, Pool 관리
│   │   ├── init.rs          # 테이블 존재 확인, 마이그레이션, 파티션 gap-filling
│   │   └── partition.rs     # 미래 파티션 생성
│   ├── api/
│   │   ├── mod.rs           # 모듈 export
│   │   ├── auth.rs          # JWT 생성 (REST + WebSocket)
│   │   ├── quotation/
│   │   │   ├── mod.rs
│   │   │   ├── candle.rs    # 시세 조회 - 초/분/일 캔들 REST
│   │   │   └── market.rs    # 페어 목록 조회 REST
│   │   ├── rest.rs          # REST API 호출 (reqwest, JWT bearer)
│   │   └── websocket.rs     # WebSocket 연결/송수신
│   ├── collector/
│   │   ├── mod.rs           # 모듈 export
│   │   ├── candles.rs       # 캔들 gap-filling (REST 조회 + DB 저장)
│   │   ├── parsers.rs       # WebSocket 메시지 파싱 (candle/ticker/trade/orderbook)
│   │   └── subscriptions.rs # WebSocket 구독 메시지 생성
│   └── cron/
│       ├── mod.rs                 # 모듈 export
│       ├── market_refresh.rs      # 페어 목록 갱신 (10분 cron)
│       ├── partition_schedule.rs  # 파티션 생성/삭제 스케줄러
│       └── partition_delete.rs    # 과거 파티션 삭제
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
url:
  rest: https://api.upbit.com
  ws: wss://api.upbit.com/websocket/v1
candle:
  units: [1m, 10m, 60m, 1d]  # REST API gap-filling할 캔들 시간 단위
  count: 200                  # REST 캔들 조회 시 count 파라미터 (최대 200)
  seconds:
    markets: [KRW-BTC]       # 1s 캔들 WebSocket 구독할 페어 (빈 배열 = 구독 안 함, gap-filling 없음)
rate_limit:
  api_calls_per_second: 5
partition:
  retain_days: 30
  retain_months: 6
  create: 3
```

## 데이터 모델

DB 컬럼명은 REST API 응답 필드명을 기준으로 매핑합니다. WebSocket 응답 필드명은 REST와 일부 다르므로 parsers에서 매핑합니다.

### REST API vs WebSocket 필드 매핑

| 데이터 타입 | REST API 필드명 | WebSocket 필드명 | DB 컬럼명 |
|------------|----------------|-----------------|----------|
| 공통 | `market` (페어 코드) | `code` | `market` |
| Candle | `candle_date_time_utc` (`yyyy-MM-dd'T'HH:mm:ss`) | `candle_date_time_utc` (`yyyy-MM-dd'T'HH:mm:ss`) | `candle_date_time_utc` |
| Trade | `trade_date_utc` (`yyyy-MM-dd`), `trade_time_utc` (`HH:mm:ss`) | `trade_date` (`yyyy-MM-dd`), `trade_time` (`HH:mm:ss`) | `trade_date_utc`, `trade_time_utc` |

> **포맷**: REST API와 WebSocket 모두 `candle_date_time_utc`는 ISO 8601 형식(`yyyy-MM-dd'T'HH:mm:ss`)으로 반환합니다.
>
> **WebSocket 포맷**: DEFAULT 형식 사용. REST API 응답 필드명을 DB 컬럼명으로 사용합니다.

## 파티션 구조

| 테이블 | 파티션 단위 | 파티션 키 | 생성 방식 |
|--------|------------|----------|----------|
| tickers | 일 | trade_date (VARCHAR) | 프로그램 생성 |
| trades | 일 | trade_date_utc (VARCHAR) | 프로그램 생성 |
| candles_seconds | 일 | candle_date_time_utc (VARCHAR) | 프로그램 생성 |
| candles_minutes | 월 | candle_date_time_utc (VARCHAR) | 프로그램 생성 |
| candles_days | 월 | candle_date_time_utc (VARCHAR) | 프로그램 생성 |
| orderbooks | - | - | 단일 테이블 |

**프로그램 시작 시:**
1. 기존 파티션 조회
2. 월별 테이블 (candles_minutes, candles_days): 마지막 파티션 ~ 현재 월 사이 빈 칸 생성 (gap-filling)
3. 일별 테이블 (tickers, trades, candles_seconds): 마지막 파티션 ~ 현재 일 사이 빈 칸 생성 (gap-filling)
4. 다음 3개월분 월 파티션 생성

**Cron (24시간마다):**
1. 다음 3개월분 월 파티션 생성 (이미 있으면 skip)
2. 과거 파티션 삭제 (tickers, trades, candles_seconds: 30일 이상 / candles_minutes, candles_days: 6개월 이상)

## 실행 시나리오

### 1. 데몬 프로그램 실행
- `cargo run` 또는 시스템 서비스로 실행
- CLI 파라미터 또는 환경 변수로 DB URL, API 키, config.yaml 경로 지정
- `config.yaml`에서 수집 설정 로드

### 2. 디비 연결
- `sqlx`를 통해 PostgreSQL 17에 연결 풀 (max 10 connections) 생성
- 연결 실패 시 에러로 종료

### 3. 테이블 초기화
- 프로그램 시작 시 핵심 테이블 (markets, tickers, trades, candles_*, orderbooks) 이 존재하는지 확인
- 초기화 안 되어 있으면 `migrations/001_initial.sql` 기반 자동 생성

### 4. 파티션 gap-filling
- 각 파티션 테이블에서 마지막 생성된 파티션 확인
- 마지막 파티션부터 현재 시점까지 누락된 파티션 테이블 자동 생성
  - `trades`, `tickers`, `candles_seconds`: 일 단위 파티션 (누락된 일자 생성)
  - `candles_minutes`, `candles_days`: 월 단위 파티션 (누락된 월 생성)

### 5. 미래 파티션 생성
- 프로그램 시작 시 다음 `partition_create` 개수 (기본 3) 개 파티션 생성
- Cron (24시간마다) 으로 계속 생성 (이미 있으면 skip)

### 6. 과거 파티션 삭제 (Cron과 함께 실행)
- `tickers` (일 단위): **30일 이상** 경과된 파티션 삭제
- `trades` (일 단위): **30일 이상** 경과된 파티션 삭제
- `candles_seconds` (일 단위): **30일 이상** 경과된 파티션 삭제
- `candles_minutes` (월 단위): **6개월 이상** 경과된 파티션 삭제
- `candles_days` (월 단위): **6개월 이상** 경과된 파티션 삭제
- 저장 공간 관리 및 쿼리 성능 유지

### 7. 페어 목록 조회
- REST API (`/v1/markets`) 로 업비트 전체 페어 목록 조회
- **프로그램 시작 시 1회** 및 **Cron으로 1시간 1회** 실행
- `markets` 테이블에 `UPSERT` (동시성 처리: `ON CONFLICT DO UPDATE`)
  - 신규 페어 추가 / 기존 페어 정보 업데이트

### 8. REST API 캔들 gap-filling (60분 Cron)
- `markets` 테이블에서 수집할 페어 목록 조회 (`candle.market_prefix`으로 필터링)
- `config.yaml`의 `candle.units` 배열에 지정된 시간 단위 (예: `[1m, 10m, 60m, 1d]`) 의 캔들 데이터를 REST API로 gap-filling
- **60분 1회** Cron 실행
- 각 페어, 단위별로 마지막 캔들 시간과 현재 시간 비교
- 누락된 캔들 확인 시 REST API (`/v1/candles/minutes/{unit}` 또는 `/v1/candles/days` for `1d`) 로 조회 (`candle.count` 개수만큼 batch)
- REST API 응답 데이터를 DB에 `UPSERT` (`INSERT ... ON CONFLICT DO UPDATE`)
- 마지막 캔들이 없으면 현재 시간 기준으로 `candle.count` 개수만큼 조회

### 9. 1s 캔들 WebSocket 구독
- `config.yaml`의 `candle.seconds.markets`에 명시된 페어만 1s 캔들 WebSocket 구독 (전체 markets 아님)
- 프로그램 시작 시 명시된 페어에 대해 `candle.1s` 구독
- 구독 중인 스트림 목록은 WebSocket `LIST_SUBSCRIPTIONS` 메서드로 조회 가능
- `1s` 캔들은 gap-filling 하지 않음 (WebSocket 실시간 수신만)

### 10. 실시간 수신 데이터 DB Upsert
- `1s` 캔들 (`candle.seconds.markets` 명시 페어) WebSocket 수신 데이터를 실시간으로 DB 저장
- `INSERT ... ON CONFLICT DO UPDATE` 방식으로 중복 방지
- WebSocket 필드명 (`code`) → REST 필드명 (`market`) 매핑하여 DB 컬럼에 저장

## 실행

```bash
# 환경 변수 설정
cp .env.example .env
# .env 편집

# 마이그레이션 실행
DATABASE_URL=... cargo sqlx migrate run

# 실행 (config.yaml 기본값: config.yaml)
cargo run

# 개발모드로 실행하여 .env 사용
cargo run --features dev

# 커스텀 config.yaml 경로
cargo run -- --config-path /path/to/config.yaml

# API 키 전달
cargo run -- --access-key YOUR_KEY --secret-key YOUR_SECRET
```
