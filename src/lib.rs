use cron::{Schedule, TimeUnitSpec};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

/// Merge multiple cron expressions into one deduped set of compiled schedules.
#[derive(Debug, Clone)]
pub struct CronUnion(Vec<CompiledCron>);

/// A compiled cron expression plus its original display form.
#[derive(Debug, Clone)]
pub struct CompiledCron {
    schedule: Schedule,
    display: String,
}

impl CompiledCron {
    pub fn schedule(&self) -> &Schedule {
        &self.schedule
    }
}

impl Display for CompiledCron {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.display)
    }
}

impl CronUnion {
    /// Build a union from cron expressions.
    ///
    /// # Errors
    ///
    /// Returns an error if any expression fails to parse as a valid cron schedule.
    pub fn new<I, S>(expressions: I) -> Result<Self, cron::error::Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut schedules = Vec::new();

        for expression in expressions {
            let display = expression
                .as_ref()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            let schedule = Schedule::from_str(&normalize_expression(&display))?;
            let candidate = CompiledCron { schedule, display };

            if schedules.iter().any(|existing: &CompiledCron| {
                schedule_is_subset(&candidate.schedule, &existing.schedule)
            }) {
                continue;
            }

            schedules.retain(|existing: &CompiledCron| {
                !schedule_is_subset(&existing.schedule, &candidate.schedule)
            });
            schedules.push(candidate);
        }

        Ok(Self(schedules))
    }

    /// Return the compiled cron expressions.
    pub fn iter(&self) -> impl Iterator<Item = &CompiledCron> {
        self.0.iter()
    }
}

/// Convenience constructor.
///
/// # Errors
///
/// Returns an error if any expression fails to parse as a valid cron schedule.
pub fn union<I, S>(expressions: I) -> Result<CronUnion, cron::error::Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    CronUnion::new(expressions)
}

fn normalize_expression(expression: &str) -> String {
    match expression {
        "@yearly" | "@annually" => "0 0 0 1 1 *".to_string(),
        "@monthly" => "0 0 0 1 * *".to_string(),
        "@weekly" => "0 0 0 * * 1".to_string(),
        "@daily" | "@midnight" => "0 0 0 * * *".to_string(),
        "@hourly" => "0 0 * * * *".to_string(),
        _ => {
            let fields: Vec<&str> = expression.split_whitespace().collect();
            match fields.as_slice() {
                [minute, hour, day_of_month, month, day_of_week] => format!(
                    "0 {minute} {hour} {day_of_month} {month} {}",
                    normalize_posix_day_of_week(day_of_week)
                ),
                _ => expression.to_string(),
            }
        }
    }
}

fn normalize_posix_day_of_week(field: &str) -> String {
    field
        .split(',')
        .map(normalize_posix_day_of_week_part)
        .collect::<Vec<_>>()
        .join(",")
}

fn normalize_posix_day_of_week_part(part: &str) -> String {
    let (base, step) = part.split_once('/').unwrap_or((part, ""));
    let normalized_base = base
        .split('-')
        .map(|value| {
            value
                .parse::<u32>()
                .map(normalize_posix_day_of_week_ordinal)
                .map(|ordinal| ordinal.to_string())
                .unwrap_or_else(|_| value.to_string())
        })
        .collect::<Vec<_>>()
        .join("-");
    if step.is_empty() {
        normalized_base
    } else {
        format!("{normalized_base}/{step}")
    }
}

const fn normalize_posix_day_of_week_ordinal(ordinal: u32) -> u32 {
    match ordinal {
        0 | 7 => 1,
        1..=6 => ordinal + 1,
        _ => ordinal,
    }
}

fn schedule_is_subset(a: &Schedule, b: &Schedule) -> bool {
    unit_is_subset(a.years(), b.years())
        && unit_is_subset(a.months(), b.months())
        && unit_is_subset(a.days_of_month(), b.days_of_month())
        && unit_is_subset(a.days_of_week(), b.days_of_week())
        && unit_is_subset(a.hours(), b.hours())
        && unit_is_subset(a.minutes(), b.minutes())
        && unit_is_subset(a.seconds(), b.seconds())
}

fn unit_is_subset<A, B>(a: &A, b: &B) -> bool
where
    A: TimeUnitSpec,
    B: TimeUnitSpec,
{
    a.iter().all(|ordinal| b.includes(ordinal))
}

#[cfg(test)]
mod lib_tests;
