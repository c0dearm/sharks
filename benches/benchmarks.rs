use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sharks::SecretShares;

fn secret_shares_generation(c: &mut Criterion) {
    let shamir = SecretShares::new(1000, 128).unwrap();
    let mut iter = shamir.iter_shares(12345).unwrap();

    c.bench_function("obtain_shares_iterator", |b| {
        b.iter(|| shamir.iter_shares(black_box(12345)))
    });
    c.bench_function("step_shares_iterator", |b| b.iter(|| iter.next()));
}

fn secret_from_shares(c: &mut Criterion) {
    let shamir = SecretShares::new(10, 128).unwrap();
    let shares: HashMap<u128, u128> = shamir.iter_shares(12345).unwrap().take(100).collect();

    c.bench_function("recover_secret", |b| {
        b.iter(|| shamir.secret_from(black_box(&shares)))
    });
}

criterion_group!(benches, secret_shares_generation, secret_from_shares);
criterion_main!(benches);
