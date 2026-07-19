use super::*;

#[test]
fn drops_redundant_crons() {
    let union = union(["0 * * * *", "*/30 * * * *"]).unwrap();

    let crons: Vec<_> = union.iter().map(ToString::to_string).collect();

    assert_eq!(crons, vec!["*/30 * * * *"]);
}

#[test]
fn keeps_distinct_crons() {
    let union = CronUnion::new(["0 9 * * *", "0 17 * * *"]).unwrap();

    let crons: Vec<_> = union.iter().map(ToString::to_string).collect();

    assert_eq!(crons, vec!["0 9 * * *", "0 17 * * *"]);
}
