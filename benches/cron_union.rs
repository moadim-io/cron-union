use criterion::{Criterion, black_box, criterion_group, criterion_main};
use cron_union::union;

fn bench_union(c: &mut Criterion) {
    c.bench_function("union_dedup_redundant_crons", |b| {
        b.iter(|| black_box(union(["0 * * * *", "*/30 * * * *"]).unwrap()))
    });

    c.bench_function("union_keeps_distinct_crons", |b| {
        b.iter(|| black_box(union(["0 9 * * *", "0 17 * * *"]).unwrap()))
    });
}

criterion_group!(benches, bench_union);
criterion_main!(benches);
