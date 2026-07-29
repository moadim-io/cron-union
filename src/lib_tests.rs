use super::*;

#[test]
fn drops_redundant_crons() {
    let union = union(["0 * * * *", "*/30 * * * *"]).unwrap();

    let crons: Vec<_> = union.iter().map(ToString::to_string).collect();

    assert_eq!(crons, vec!["*/30 * * * *"]);
}

#[test]
fn allows_an_empty_union() {
    let union = CronUnion::new(std::iter::empty::<&str>()).unwrap();

    assert!(union.iter().next().is_none());
}

#[test]
fn removes_supersets_when_a_more_specific_cron_arrives_later() {
    let union = CronUnion::new(["*/30 * * * *", "0 * * * *"]).unwrap();

    let crons: Vec<_> = union.iter().map(ToString::to_string).collect();

    assert_eq!(crons, vec!["*/30 * * * *"]);
}

#[test]
fn keeps_distinct_crons() {
    let union = CronUnion::new(["0 9 * * *", "0 17 * * *"]).unwrap();

    let crons: Vec<_> = union.iter().map(ToString::to_string).collect();

    assert_eq!(crons, vec!["0 9 * * *", "0 17 * * *"]);
}

#[test]
fn preserves_six_field_expressions_and_exposes_the_schedule() {
    let union = CronUnion::new(["0 */5 * * * *"]).unwrap();
    let cron = union.iter().next().unwrap();

    assert_eq!(cron.to_string(), "0 */5 * * * *");
    assert_eq!(cron.schedule().to_string(), "0 */5 * * * *");
}

#[test]
fn accepts_posix_zero_based_days_of_week_in_five_field_crons() {
    let union = CronUnion::new(["0 18 * * 0-4"]).unwrap();
    let cron = union.iter().next().unwrap();

    assert_eq!(cron.to_string(), "0 18 * * 0-4");
    assert_eq!(cron.schedule().to_string(), "0 0 18 * * 1-5");
}

#[test]
fn preserves_posix_weekday_semantics_for_subset_checks() {
    let union = CronUnion::new(["0 18 * * 1-5", "0 18 * * 1"]).unwrap();
    let crons: Vec<_> = union.iter().map(ToString::to_string).collect();

    assert_eq!(crons, vec!["0 18 * * 1-5"]);
}

#[test]
fn keeps_distinct_crons_when_seconds_differ() {
    let union = CronUnion::new(["0 */5 * * * *", "30 */5 * * * *"]).unwrap();

    let crons: Vec<_> = union.iter().map(ToString::to_string).collect();

    assert_eq!(crons, vec!["0 */5 * * * *", "30 */5 * * * *"]);
}

#[test]
fn keeps_distinct_crons_when_minutes_differ() {
    let union = CronUnion::new(["0 0 9 * * *", "0 1 9 * * *"]).unwrap();

    let crons: Vec<_> = union.iter().map(ToString::to_string).collect();

    assert_eq!(crons, vec!["0 0 9 * * *", "0 1 9 * * *"]);
}

#[test]
fn keeps_distinct_crons_across_time_units() {
    for (left, right) in [
        ("0 0 9 1 * *", "0 0 9 2 * *"),
        ("0 0 9 * 1 *", "0 0 9 * 2 *"),
        ("0 0 9 * * 1", "0 0 9 * * 2"),
        ("0 0 9 * * * 2024", "0 0 9 * * * 2025"),
    ] {
        let union = CronUnion::new([left, right]).unwrap();
        let crons: Vec<_> = union.iter().map(ToString::to_string).collect();

        assert_eq!(crons, vec![left, right]);
    }
}

#[test]
fn supports_at_cron_aliases() {
    let daily = CronUnion::new(["@daily"]).unwrap();
    let weekly = CronUnion::new(["@weekly"]).unwrap();
    let monthly = CronUnion::new(["@monthly"]).unwrap();
    let yearly = CronUnion::new(["@yearly"]).unwrap();

    assert_eq!(
        daily.iter().next().unwrap().schedule().to_string(),
        "0 0 0 * * *"
    );
    assert_eq!(
        weekly.iter().next().unwrap().schedule().to_string(),
        "0 0 0 * * 1"
    );
    assert_eq!(
        monthly.iter().next().unwrap().schedule().to_string(),
        "0 0 0 1 * *"
    );
    assert_eq!(
        yearly.iter().next().unwrap().schedule().to_string(),
        "0 0 0 1 1 *"
    );
}

#[test]
fn returns_an_error_for_invalid_expressions() {
    assert!(union(["not a cron"]).is_err());
}
