use chrono::Utc;
use cron::Schedule;
use std::str::FromStr;
use tokio::time::Instant;

pub fn next_cron_instant(cron_expr: Option<&str>, default: Instant) -> Instant {
    let cron_expr = cron_expr.unwrap_or("*/10 * * * *");
    match Schedule::from_str(cron_expr) {
        Ok(schedule) => {
            schedule.upcoming(Utc)
                .next()
                .map(|dt| {
                    let now = Utc::now();
                    let diff = dt.signed_duration_since(now);
                    let secs = diff.num_seconds().max(0) as u64;
                    let nanos = diff.num_microseconds().unwrap_or(0) % 1_000_000;
                    Instant::now() + std::time::Duration::new(secs, (nanos as u32) * 1000)
                })
                .unwrap_or(default)
        }
        Err(_) => default,
    }
}
