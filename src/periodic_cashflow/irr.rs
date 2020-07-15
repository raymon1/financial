use crate::common::{find_root::find_root, utils};
use crate::periodic_cashflow::npv::npv;

/// Calculates the internal rate of return for a series of cash flows occurring at regular interval represented by the numbers in values.
///
/// # Example
/// ```
/// let cf = [-500., 100., 100., 100., 100., 100.];
/// let guess = Some(0.);
/// let cf_irr = financial::irr(&cf, guess);
/// ```
pub fn irr(values: &[f64], guess: Option<f64>) -> Result<f64, &'static str> {
    let values = utils::trim_zeros(&values);

    match utils::validate_cashflow_values(values) {
        Err(x) => return Err(x),
        Ok(()) => {}
    }

    let f_npv = |x: f64| npv(x, values);
    match find_root(guess, f_npv) {
        Some(ans) => Ok(ans),
        None => Err("could't find irr for the values provided"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::PRECISION;

    #[test]
    fn irr_works_different_guess_sign() {
        let cf = [-500., 100., 100., 100., 100.];
        let guess = Some(0.);
        let precision = (irr(&cf, guess).unwrap() - -0.08364541746615000000000000000000).abs();
        assert!(
            precision <= PRECISION,
            format!(
                "IRR of {}, exceeded IRR precision threshold, {}",
                irr(&cf, guess).unwrap(),
                precision
            )
        );
    }

    #[test]
    fn irr_works() {
        let cf = [-500., 100., 100., 100., 100.];
        let guess = Some(-0.);
        let precision = (irr(&cf, guess).unwrap() - -0.08364541746615000000000000000000).abs();
        assert!(
            precision <= PRECISION,
            format!(
                "exceeded {} IRR precision threshold, {}",
                irr(&cf, guess).unwrap(),
                precision
            )
        );
    }

    #[test]
    fn irr_works_2() {
        let cf = [-500., 100., 100., 100., 100., 100.];
        let guess = Some(0.);
        assert_eq!(irr(&cf, guess).unwrap(), 0.0);
    }
}
