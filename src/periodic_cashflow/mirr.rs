use crate::common::utils;

/// Returns the modified internal rate of return for a series of periodic cash flows.
/// MIRR considers both the cost of the investment and the interest received on reinvestment of cash.
/// 
/// # Example
/// ```
/// let cf = [-1000., 100., 200., 300., 400., 400., 400.];
/// let finance_rate = 0.1;
/// let reinvest_rate = 0.1;
/// let mirr = financial::mirr(&cf, finance_rate, reinvest_rate);
/// ```
pub fn mirr(values: &[f64], finance_rate: f64, reinvest_rate: f64) -> f64 {
    let values = utils::trim_end_zeros(values);

    let negative_cf_pv : f64 = utils::powers(1. + finance_rate, values.len(), true)
    .iter()
    .zip(values.iter())
    .filter(|(_r, &v)| v < 0.)
    .map(|(r, v)| *v / *r)
    .sum();

    let positive_cf_fv : f64 = utils::powers(1. + reinvest_rate, values.len(), true)
    .iter()
    .zip(values.iter().rev())
    .filter(|(_r, &v)| v > 0.)
    .map(|(r, v)| v * r)
    .sum();

    (positive_cf_fv / -negative_cf_pv).powf(1. / (values.len() - 1) as f64) - 1.
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::PRECISION;

    #[test]
    fn mirr_works() {
        let cf = [-1000., 100., 200., 300., 400., 400., 400.];
        let finance_rate = 0.1;
        let reinvest_rate = 0.1;
        let mirr = mirr(&cf, finance_rate, reinvest_rate);
        
        let ans = 0.138453832579;
        assert!(
           (mirr - ans).abs() <= PRECISION,
            format!(
                "ans is {} got {}",
                ans,
                mirr
            )
        );
    }

    #[test]
    fn mirr_works_negative_cashflow() {
        let cf = [-100_000., 18_000., -50_000., 25_000., 25_000., 225_000.];
        let finance_rate = 0.05;
        let reinvest_rate = 0.1;
        let mirr = mirr(&cf, finance_rate, reinvest_rate);
        
        let ans = 0.16288556821502476;
        assert!(
           (mirr - ans).abs() <= PRECISION,
            format!(
                "ans is {} got {}",
                ans,
                mirr
            )
        );        
    }

    #[test]
    fn mirr_works_no_negative_cashflow() {
        let cf = [100_000., 18_000., 50_000., 25_000., 25_000., 225_000.];
        let finance_rate = 0.05;
        let reinvest_rate = 0.1;
        let mirr = mirr(&cf, finance_rate, reinvest_rate);
        
        let ans = f64::INFINITY;
        assert!(
           mirr.is_infinite(),
            format!(
                "ans is {} got {}",
                ans,
                mirr
            )
        );        
    }
}
