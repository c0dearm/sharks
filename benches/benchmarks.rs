use criterion::{black_box, criterion_group, criterion_main, Criterion};

use sharks::Sharks;

fn dealer(c: &mut Criterion) {
    let sharks = Sharks(255);
    let mut dealer = sharks.dealer(&[1]);

    c.bench_function("obtain_shares_dealer", |b| {
        b.iter(|| sharks.dealer(black_box(&[1])))
    });
    c.bench_function("step_shares_dealer", |b| b.iter(|| dealer.next()));
}

fn recover(c: &mut Criterion) {
    let sharks = Sharks(255);
    let shares = sharks.dealer(&[1]).take(255).collect();

    c.bench_function("recover_secret", |b| {
        b.iter(|| sharks.recover(black_box(&shares)))
    });
}

criterion_group!(benches, dealer, recover);
criterion_main!(benches);
