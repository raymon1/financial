//! Benchmarks

extern crate criterion;

use chrono::{DateTime, NaiveDate, Utc};
use criterion::{criterion_group, criterion_main, Criterion};
use financial;

fn bench_npv_100_value(c: &mut Criterion) {
    c.bench_function("bench_npv_100_value", |b| {
        let x: [f64; 100] = [100.; 100];
        b.iter(|| financial::npv(0.1, &x));
    });
}

fn bench_npv_10000_value(c: &mut Criterion) {
    c.bench_function("bench_npv_10000_value", |b| {
        let x: [f64; 10000] = [100.; 10000];
        b.iter(|| financial::npv(0.1, &x));
    });
}

fn bench_npv_10000_value_zero_rate(c: &mut Criterion) {
    c.bench_function("bench_npv_10000_value_zero_rate", |b| {
        let x: [f64; 10000] = [100.; 10000];
        b.iter(|| financial::npv(0., &x));
    });
}

fn bench_irr(c: &mut Criterion) {
    let cf = [-500., 100., 100., 100., 100., 100.];
    c.bench_function("bench_irr", |b| {
        b.iter(|| financial::irr(&cf, Some(0.)));
    });
}

fn bench_xnpv(c: &mut Criterion) {
    let cf = [-500., 100., 100., 100., 100., 100.];
    let dates = [
        DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2016, 7, 8).and_hms(0, 0, 0), Utc),
        DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2017, 7, 8).and_hms(0, 0, 0), Utc),
        DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2018, 7, 8).and_hms(0, 0, 0), Utc),
        DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2019, 7, 8).and_hms(0, 0, 0), Utc),
        DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2020, 7, 8).and_hms(0, 0, 0), Utc),
        DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2021, 7, 8).and_hms(0, 0, 0), Utc),
    ];
    c.bench_function("bench_xnpv", |b| {
        b.iter(|| financial::xnpv(0.1, &cf, &dates));
    });
}

fn bench_xirr(c: &mut Criterion) {
    let cf = [-500., 100., 100., 100., 100., 100.];
    let dates = [
        DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2016, 7, 8).and_hms(0, 0, 0), Utc),
        DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2017, 7, 8).and_hms(0, 0, 0), Utc),
        DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2018, 7, 8).and_hms(0, 0, 0), Utc),
        DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2019, 7, 8).and_hms(0, 0, 0), Utc),
        DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2020, 7, 8).and_hms(0, 0, 0), Utc),
        DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2021, 7, 8).and_hms(0, 0, 0), Utc),
    ];
    c.bench_function("bench_xirr", |b| {
        b.iter(|| financial::xirr(&cf, &dates, None));
    });
}

criterion_group!(
    benches,
    bench_npv_100_value,
    bench_npv_10000_value,
    bench_npv_10000_value_zero_rate,
    bench_irr,
    bench_xnpv,
    bench_xirr
);

criterion_main!(benches);
