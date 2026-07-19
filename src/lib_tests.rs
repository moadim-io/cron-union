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
