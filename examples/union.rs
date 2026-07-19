use chrono::Utc;
use cron_union::union;

fn main() {
    let start = Utc::now();
    let schedule = union(["*/5 * * * *", "0 * * * *"], start).unwrap();

    for time in schedule.iter().take(5) {
        println!("{time}");
    }
}
