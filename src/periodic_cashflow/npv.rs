use crate::common::utils;

/// Calculates the net present value of an investment by using a discount rate and a series of future payments
/// (negative values) and income (positive values).
///
/// # Examples
///
/// ```
/// let npv = financial::npv(0.1, &[-1000., 500., 500., 500.]);
/// assert_eq!(npv, 221.29635953828267);
/// ```
// pre calculating powers for performance
pub fn npv(rate: f64, values: &[f64]) -> f64 {
    if rate == 0.0 {
        return values.iter().sum();
    }

    utils::powers(1. + rate, values.len(), false)
        .iter()
        .zip(values.iter())
        .map(|(p, v)| v / *p)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn npv_with_zero_rate() {
        let x: [f64; 10000] = [100.; 10000];
        assert_eq!(npv(0., &x), x.iter().sum());
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
        assert_eq!(npv(rate, &cf), npv_slow(&rate, &cf));
    }
}
