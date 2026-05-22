CREATE TABLE markets (
    market VARCHAR(30) PRIMARY KEY,
    korean_name TEXT,
    english_name TEXT,
    market_event_warning BOOLEAN DEFAULT FALSE,
    market_event_caution_price_fluctuations BOOLEAN DEFAULT FALSE,
    market_event_caution_trading_volume_soloing BOOLEAN DEFAULT FALSE,
    market_event_caution_deposit_amount_soloing BOOLEAN DEFAULT FALSE,
    market_event_caution_global_price_differences BOOLEAN DEFAULT FALSE,
    market_event_caution_concentration_of_small_accounts BOOLEAN DEFAULT FALSE
);

CREATE TABLE tickers (
    market VARCHAR(30) NOT NULL,
    trade_date VARCHAR(8) NOT NULL,
    trade_time VARCHAR(6) NOT NULL,
    trade_date_kst VARCHAR(8) NOT NULL,
    trade_time_kst VARCHAR(6) NOT NULL,
    trade_timestamp BIGINT NOT NULL,
    opening_price DOUBLE PRECISION NOT NULL,
    high_price DOUBLE PRECISION NOT NULL,
    low_price DOUBLE PRECISION NOT NULL,
    trade_price DOUBLE PRECISION NOT NULL,
    prev_closing_price DOUBLE PRECISION NOT NULL,
    change VARCHAR(10) NOT NULL,
    change_price DOUBLE PRECISION NOT NULL,
    change_rate DOUBLE PRECISION NOT NULL,
    signed_change_price DOUBLE PRECISION NOT NULL,
    signed_change_rate DOUBLE PRECISION NOT NULL,
    trade_volume DOUBLE PRECISION NOT NULL,
    acc_trade_price DOUBLE PRECISION NOT NULL,
    acc_trade_price_24h DOUBLE PRECISION,
    acc_trade_volume DOUBLE PRECISION NOT NULL,
    acc_trade_volume_24h DOUBLE PRECISION,
    highest_52_week_price DOUBLE PRECISION,
    highest_52_week_date VARCHAR(10),
    lowest_52_week_price DOUBLE PRECISION,
    lowest_52_week_date VARCHAR(10),
    timestamp BIGINT NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
) PARTITION BY RANGE (trade_date);

CREATE TABLE tickers_y2026m01d01 PARTITION OF tickers FOR VALUES FROM ('20260101') TO ('20260102');

CREATE TABLE trades (
    market VARCHAR(30) NOT NULL,
    trade_date_utc VARCHAR(10) NOT NULL,
    trade_time_utc VARCHAR(10) NOT NULL,
    trade_price DOUBLE PRECISION NOT NULL,
    trade_volume DOUBLE PRECISION NOT NULL,
    sequential_id BIGINT NOT NULL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
) PARTITION BY RANGE (trade_date_utc);

CREATE TABLE trades_y2026m01d01 PARTITION OF trades FOR VALUES FROM ('2026-01-01') TO ('2026-01-02');

CREATE TABLE candles_seconds (
    market VARCHAR(30) NOT NULL,
    candle_date_time_utc VARCHAR(20) NOT NULL,
    candle_date_time_kst VARCHAR(20) NOT NULL,
    opening_price DOUBLE PRECISION NOT NULL,
    high_price DOUBLE PRECISION NOT NULL,
    low_price DOUBLE PRECISION NOT NULL,
    trade_price DOUBLE PRECISION NOT NULL,
    timestamp BIGINT NOT NULL,
    candle_acc_trade_price DOUBLE PRECISION NOT NULL,
    candle_acc_trade_volume DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
) PARTITION BY RANGE (candle_date_time_utc);

CREATE TABLE candles_seconds_y2026m01d01 PARTITION OF candles_seconds FOR VALUES FROM ('2026-01-01T00:00:00') TO ('2026-01-02T00:00:00');

CREATE TABLE candles_minutes (
    market VARCHAR(30) NOT NULL,
    candle_date_time_utc VARCHAR(20) NOT NULL,
    candle_date_time_kst VARCHAR(20) NOT NULL,
    opening_price DOUBLE PRECISION NOT NULL,
    high_price DOUBLE PRECISION NOT NULL,
    low_price DOUBLE PRECISION NOT NULL,
    trade_price DOUBLE PRECISION NOT NULL,
    timestamp BIGINT NOT NULL,
    candle_acc_trade_price DOUBLE PRECISION NOT NULL,
    candle_acc_trade_volume DOUBLE PRECISION NOT NULL,
    unit INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
) PARTITION BY RANGE (candle_date_time_utc);

CREATE TABLE candles_minutes_y2026m01 PARTITION OF candles_minutes FOR VALUES FROM ('2026-01-01T00:00:00') TO ('2026-02-01T00:00:00');

CREATE TABLE candles_days (
    market VARCHAR(30) NOT NULL,
    candle_date_time_utc VARCHAR(11) NOT NULL,
    candle_date_time_kst VARCHAR(11) NOT NULL,
    opening_price DOUBLE PRECISION NOT NULL,
    high_price DOUBLE PRECISION NOT NULL,
    low_price DOUBLE PRECISION NOT NULL,
    trade_price DOUBLE PRECISION NOT NULL,
    timestamp BIGINT NOT NULL,
    candle_acc_trade_price DOUBLE PRECISION NOT NULL,
    candle_acc_trade_volume DOUBLE PRECISION NOT NULL,
    prev_closing_price DOUBLE PRECISION NOT NULL,
    change_price DOUBLE PRECISION NOT NULL,
    change_rate DOUBLE PRECISION NOT NULL,
    converted_trade_price DOUBLE PRECISION,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
) PARTITION BY RANGE (candle_date_time_utc);

CREATE TABLE orderbooks (
    market VARCHAR(30) NOT NULL,
    timestamp BIGINT NOT NULL,
    total_ask_size DOUBLE PRECISION NOT NULL,
    total_bid_size DOUBLE PRECISION NOT NULL,
    orderbook_units JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
CREATE INDEX idx_candles_seconds_market ON candles_seconds(market, candle_date_time_utc);
CREATE INDEX idx_candles_minutes_market ON candles_minutes(market, candle_date_time_utc);
CREATE INDEX idx_candles_minutes_unit ON candles_minutes(market, candle_date_time_utc, unit);
CREATE INDEX idx_candles_days_market ON candles_days(market, candle_date_time_utc);
CREATE INDEX idx_orderbooks_market ON orderbooks(market);
CREATE INDEX idx_orderbooks_timestamp ON orderbooks(timestamp);
