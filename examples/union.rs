use cron_union::union;

fn main() {
    let crons = union(["0 * * * *", "*/30 * * * *"]).unwrap();

    for cron in crons.iter() {
        println!("{cron}");
    }
}
