
//! Benchmarks.
//!
//! Note: these don't work on 16-bit machines.

use criterion::{
    black_box, criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion,
    PlotConfiguration,
};
use ibig::{modular::ModuloRing, ops::DivRem, ubig, UBig};
use rand::prelude::*;
use std::fmt::Write;

fn random_ubig<R>(bits: usize, rng: &mut R) -> UBig
where
    R: Rng + ?Sized,
{
    rng.gen_range(ubig!(1) << (bits - 1)..ubig!(1) << bits)
}

fn bench_add(criterion: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(1);
    let mut group = criterion.benchmark_group("add");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for log_bits in 1..=6 {
        let bits = 10usize.pow(log_bits);
        let a = random_ubig(bits, &mut rng);
        let b = random_ubig(bits, &mut rng);
        group.bench_with_input(BenchmarkId::from_parameter(bits), &bits, |bencher, _| {
            bencher.iter(|| black_box(&a) + black_box(&b))
        });
    }

    group.finish();
}

fn bench_sub(criterion: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(1);
    let mut group = criterion.benchmark_group("sub");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for log_bits in 1..=6 {
        let bits = 10usize.pow(log_bits);
        let a = random_ubig(bits, &mut rng);
        let b = random_ubig(bits, &mut rng);
        let c = a + &b;
        group.bench_with_input(BenchmarkId::from_parameter(bits), &bits, |bencher, _| {
            bencher.iter(|| black_box(&c) - black_box(&b))
        });
    }

    group.finish();
}

fn bench_mul(criterion: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(1);
    let mut group = criterion.benchmark_group("mul");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for log_bits in 1..=6 {
        let bits = 10usize.pow(log_bits);
        let a = random_ubig(bits, &mut rng);
        let b = random_ubig(bits, &mut rng);
        group.bench_with_input(BenchmarkId::from_parameter(bits), &bits, |bencher, _| {
            bencher.iter(|| black_box(&a) * black_box(&b))
        });
    }

    group.finish();
}

fn bench_div(criterion: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(1);
    let mut group = criterion.benchmark_group("div");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for log_bits in 1..=6 {
        let bits = 10usize.pow(log_bits);
        let a = random_ubig(2 * bits, &mut rng);
        let b = random_ubig(bits, &mut rng);
        group.bench_with_input(BenchmarkId::from_parameter(bits), &bits, |bencher, _| {
            bencher.iter(|| black_box(&a).div_rem(black_box(&b)))
        });
    }

    group.finish();
}

fn bench_gcd(criterion: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(1);
    let mut group = criterion.benchmark_group("gcd");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for log_bits in 1..=5 {
        let bits = 10usize.pow(log_bits);
        let a = random_ubig(bits, &mut rng);
        let b = random_ubig(bits, &mut rng);
        group.bench_with_input(BenchmarkId::from_parameter(bits), &bits, |bencher, _| {
            bencher.iter(|| black_box(&a).gcd(black_box(&b)))
        });
    }

    group.finish();

    let mut group = criterion.benchmark_group("extended_gcd");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for log_bits in 1..=5 {
        let bits = 10usize.pow(log_bits);
        let a = random_ubig(bits, &mut rng);
        let b = random_ubig(bits, &mut rng);
        group.bench_with_input(BenchmarkId::from_parameter(bits), &bits, |bencher, _| {
            bencher.iter(|| black_box(&a).extended_gcd(black_box(&b)))
        });
    }

    group.finish();
}

fn bench_to_hex(criterion: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(1);
    let mut group = criterion.benchmark_group("to_hex");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for log_bits in 1..=6 {
        let bits = 10usize.pow(log_bits);
        let a = random_ubig(bits, &mut rng);
        let mut out = String::with_capacity(bits / 4 + 1);
        group.bench_with_input(BenchmarkId::from_parameter(bits), &bits, |bencher, _| {
            bencher.iter(|| {
                out.clear();
                write!(&mut out, "{:x}", black_box(&a)).unwrap();
                out.len()
            })
        });
    }