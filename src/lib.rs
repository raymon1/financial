// #![feature(test)]

//! # Financial
//!
//! `Financial` is a collection of finance calculations memicking some of Excel Financial Functions interface.



/// Calculates the future value of an investment based on a constant interest rate.
/// You can use FV with either periodic, constant payments, or a single lump sum payment.
/// 
/// # Examples
///
/// ```
/// let fv = financial::fv(&0.1, &5.0, &Some(100.0), &Some(1000.0), &Some(false));
/// assert_eq!(fv, -2221.020000000001);
/// ```
pub fn fv(rate: &f64, nper: &f64, pmt: &Option<f64>, pv: &Option<f64>, pmt_at_begining: &Option<bool>) -> f64 {
    let factor = |r| f64::powf(1.0 + r, f64::from(*nper));

    let pmt = pmt.unwrap_or_else(|| { 0.0 });
    let pv = pv.unwrap_or_else(|| { 0.0 });
   
    if *rate == 0.0 { 
        return - (pv + pmt * nper);
    } else {          
        let factor = factor(rate);
        let pmt_at_begining = if pmt_at_begining.unwrap_or_else(|| { false }) { 1.0 } else { 0.0 };
        
        - pv * factor 
        - pmt * (1.0 + rate * pmt_at_begining) / rate * (factor - 1.0)        
    }
}

/// Calculates the present value of a loan or an investment, based on a constant interest rate.
/// You can use PV with either periodic, constant payments (such as a mortgage or other loan),
/// or a future value that's your investment goal.
/// 
/// # Examples
///
/// ```
/// let fv = financial::fv(&0.1, &5.0, &Some(100.0), &Some(1000.0), &Some(false));
/// assert_eq!(fv, -2221.020000000001);
/// ```
pub fn pv(rate: &f64, nper: &f64, pmt: &Option<f64>, fv: &Option<f64>, pmt_at_begining: &Option<bool>) -> f64 {
    let pmt = pmt.unwrap_or_else(|| { 0. });
    let fv = fv.unwrap_or_else(|| { 0. });
    
    if *rate == 0.0 {
        return -(fv + pmt * nper);
    }
    else {
        let pmt_at_begining = if pmt_at_begining.unwrap_or_else(|| { false }) { 1. } else { 0. };
        let temp = f64::powf(1. + rate, *nper);
        let factor = (1. + rate * pmt_at_begining) * (temp - 1.) / rate; 
        -(fv + pmt * factor) / temp 
    }
}

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

    (1..values.len())
    .fold(vec![1. + rate], |mut pows, _| {
        pows.push(pows.last().unwrap() * (1. + rate)); 
        pows
    })
    .iter()
    .zip(values.iter())
    .map(|(p, v)| { v / *p })
    .sum()
}

// extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
//    use test::Bencher;

    #[test]
    fn fv_works_when_pmt_at_end_of_period() {
        assert_eq!(fv(&0.1, &5.0, &Some(100.0), &Some(1000.0), &Some(false)), -2221.020000000001);
    }

    #[test]
    fn fv_works_when_pmt_at_beginning() {
        assert_eq!(fv(&0.1, &5.0, &Some(100.0), &Some(1000.0), &Some(true)), -2282.071000000001);
    }

    #[test]
    fn fv_works_with_pmt_only() {
        assert_eq!(fv(&0.1, &5.0, &Some(100.0), &None, &Some(false)), -610.5100000000006);
    }

    #[test]
    fn fv_works_with_zero_rate() {
        assert_eq!(fv(&0.0, &5.0, &Some(100.0), &Some(1000.0), &None), -1500.0);
    }

    #[test]
    fn fv_dummy() {
        let rates = [0.1, 0.1, 0.1, 0.0];
        let nper = 5.0;
        let pmt = Some(100.0);
        let pvs = [Some(1000.0), Some(1000.0), None, Some(1000.0)];
        let pmt_at_begining = [Some(false), Some(true), Some(false), None];
        
        let results : Vec<f64> = rates.iter()
        .zip(pvs.iter())
        .zip(pmt_at_begining.iter())
        .map(|(rpv, a)| fv(&rpv.0, &nper, &pmt, &rpv.1, &a))
        .collect();

        assert_eq!(results, [-2221.020000000001, -2282.071000000001, -610.5100000000006, -1500.0]);
    }

    #[test]
    fn pv_works_when_pmt_at_end_of_period() {
        assert_eq!(pv(&0.1, &5.0, &Some(100.0), &Some(1000.0), &Some(false)), -1000.0000000000001);
    }

    #[test]
    fn pv_works_when_pmt_at_beginning() {
        assert_eq!(pv(&0.1, &5.0, &Some(100.0), &Some(1000.0), &Some(true)), -1037.90786769408449);
    }

    #[test]
    fn pv_works_with_pmt_only() {
        assert_eq!(pv(&0.1, &5.0, &Some(100.0), &None, &Some(false)), -379.07867694084507);
    }

    #[test]
    fn pv_works_with_zero_rate() {
        assert_eq!(pv(&0.0, &5.0, &Some(100.0), &Some(1000.0), &None), -1500.0);
    }

    #[test]
    fn npv_with_zero_rate() {
        let x: [f64; 10000] = [100.; 10000];
        assert_eq!(npv(&0., &x), x.iter().sum());        
    }

    // used for benchmarking
    fn npv_slow(rate: &f64, values: &[f64]) -> f64 {
        (0..values.len())
        .zip(values.iter())
        .map(|(n, v)| { v / f64::powf(1.0 + rate, 1. + n as f64) })
        .sum()
    }

    #[test]
    fn npv_returns_same_as_npv_slow () {
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
