mod nts;
pub mod records;
pub mod nts_protocol;
pub mod byteorder;
use std::sync::Arc;
use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;
use crate::nts::nts::obtain_nts_time;

fn test_nts(){
    obtain_nts_time();
}

fn test_nts_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("test_nts_bench.rs");
    group
        .significance_level(0.1)
        .sample_size(10)
        .measurement_time(Duration::from_secs(240));

    group.bench_function("test_nts", |b| {
        b.iter(move || test_nts());
    });

    group.finish();
}

criterion_group!(
    benches,
	test_nts_bench
);
criterion_main!(benches);