use chrono::{DateTime, Utc};
use cron::Schedule;
use std::str::FromStr;

/// Merge multiple cron expressions into one deduped stream of fire times.
#[derive(Debug, Clone)]
pub struct CronUnion {
    schedules: Vec<ScheduleState>,
}

#[derive(Debug, Clone)]
struct ScheduleState {
    schedule: Schedule,
    next: Option<DateTime<Utc>>,
}

impl CronUnion {
    /// Build a union from cron expressions.
    pub fn new<I, S>(expressions: I, start: DateTime<Utc>) -> Result<Self, cron::error::Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut schedules = Vec::new();

        for expression in expressions {
            let schedule = Schedule::from_str(&normalize_expression(expression.as_ref()))?;
            let next = schedule.after(&start).next();
            schedules.push(ScheduleState { schedule, next });
        }

        Ok(Self { schedules })
    }

    /// Return the next deduped fire times.
    pub fn iter(self) -> CronUnionIter {
        CronUnionIter { inner: self }
    }
}

/// Convenience constructor.
pub fn union<I, S>(expressions: I, start: DateTime<Utc>) -> Result<CronUnion, cron::error::Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    CronUnion::new(expressions, start)
}

fn normalize_expression(expression: &str) -> String {
    match expression.split_whitespace().count() {
        5 => format!("0 {expression}"),
        _ => expression.to_string(),
    }
}

pub struct CronUnionIter {
    inner: CronUnion,
}

impl Iterator for CronUnionIter {
    type Item = DateTime<Utc>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self
            .inner
            .schedules
            .iter()
            .filter_map(|state| state.next)
            .min()?;

        for state in &mut self.inner.schedules {
            if state.next == Some(next) {
                state.next = state.schedule.after(&next).next();
            }
        }

        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn dedupes_overlapping_fire_times() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let union = union(["0 * * * *", "*/30 * * * *"], start).unwrap();

        let times: Vec<_> = union.iter().take(4).collect();

        assert_eq!(
            times,
            vec![
                Utc.with_ymd_and_hms(2024, 1, 1, 0, 30, 0).unwrap(),
                Utc.with_ymd_and_hms(2024, 1, 1, 1, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2024, 1, 1, 1, 30, 0).unwrap(),
                Utc.with_ymd_and_hms(2024, 1, 1, 2, 0, 0).unwrap(),
            ]
        );
    }

    #[test]
    fn accepts_multiple_expressions() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let union = CronUnion::new(["0 9 * * *", "0 17 * * *"], start).unwrap();

        let times: Vec<_> = union.iter().take(2).collect();

        assert_eq!(
            times,
            vec![
                Utc.with_ymd_and_hms(2024, 1, 1, 9, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2024, 1, 1, 17, 0, 0).unwrap(),
            ]
        );
    }
}
