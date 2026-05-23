use chrono::{DateTime, Utc};
use cron::Schedule;
use std::str::FromStr;
use std::time::Duration;

pub fn cron_expression_to_interval(cron_expr: &str) -> Option<Duration> {
    let schedule = Schedule::from_str(cron_expr).ok()?;
    let mut times = schedule.upcoming(Utc).take(3).collect::<Vec<DateTime<Utc>>>();

    if times.len() < 2 {
        return None;
    }

    times.sort();
    let diff = times[1] - times[0];
    let total_secs = diff.num_seconds().max(1) as u64;
    Some(Duration::from_secs(total_secs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_every_10_minutes() {
        let interval = cron_expression_to_interval("*/10 * * * *").unwrap();
        assert_eq!(interval.as_secs(), 600);
    }

    #[test]
    fn test_every_1_hour() {
        let interval = cron_expression_to_interval("0 * * * *").unwrap();
        assert_eq!(interval.as_secs(), 3600);
    }

    #[test]
    fn test_every_24_hours() {
        let interval = cron_expression_to_interval("0 0 * * *").unwrap();
        assert_eq!(interval.as_secs(), 86400);
    }
}
