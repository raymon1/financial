// #![feature(test)]
use crate::common::utils;

/// Calculates the net present value of an investment by using a discount rate and a series of future payments
/// (negative values) and income (positive values).
///
/// # Examples
///
/// ```
/// let npv = financial::npv(&0.1, &[-1000., 500., 500., 500.]);
/// assert_eq!(npv, 221.29635953828267);
/// ```
// pre calculating powers for performance
pub fn npv(rate: &f64, values: &[f64]) -> f64 {
    if *rate == 0.0 {
        return values.iter().sum();
    }

    utils::powers(&(1. + rate), &values.len(), false)
        .iter()
        .zip(values.iter())
        .map(|(p, v)| v / *p)
        .sum()
}

// extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    //    use test::Bencher;

    #[test]
    fn npv_with_zero_rate() {
        let x: [f64; 10000] = [100.; 10000];
        assert_eq!(npv(&0., &x), x.iter().sum());
    }

    // used for benchmarking
    fn npv_slow(rate: &f64, values: &[f64]) -> f64 {
        (0..values.len())
            .zip(values.iter())
            .map(|(n, v)| v / f64::powf(1.0 + rate, 1. + n as f64))
            .sum()
    }

    #[test]
    fn npv_returns_same_as_npv_slow() {
        let cf = [-1000., 100., 100., 100.];
        let rate = 0.1;
        assert_eq!(npv(&rate, &cf), npv_slow(&rate, &cf));
    }

    // commenting benchmarking as it's included in the nightly build only
    // #[bench]
    // fn bench_npv_low_count(b: &mut Bencher) {
    //     let x: [f64; 100] = [100.; 100];
    //     b.iter(|| npv(&0.1, &x));
    // }

    // #[bench]
    // fn bench_npv_slow_low_count(b: &mut Bencher) {
    //     let x: [f64; 100] = [100.; 100];
    //     b.iter(|| npv_slow(&0.1, &x));
    // }

    // #[bench]
    // fn bench_npv(b: &mut Bencher) {
    //     let x: [f64; 10000] = [100.; 10000];
    //     b.iter(|| npv(&0.1, &x));
    // }

    // #[bench]
    // fn bench_npv_zero_rate(b: &mut Bencher) {
    //     let x: [f64; 10000] = [100.; 10000];
    //     b.iter(|| npv(&0., &x));
    // }

    // #[bench]
    // fn bench_npv_slow(b: &mut Bencher) {
    //     let x: [f64; 10000] = [100.; 10000];
    //     b.iter(|| npv_slow(&0.1, &x));
    // }

    // #[bench]
    // fn bench_npv_slow_zero_rate(b: &mut Bencher) {
    //     let x: [f64; 10000] = [100.; 10000];
    //     b.iter(|| npv_slow(&0., &x));
    // }
}
