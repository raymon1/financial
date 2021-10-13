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

    // IRR is calculated by the bisection-search of a root of a polynomial.
    // For it to work, we need to establish search bounds; we determine them by probing some values of the argument,
    // starting from the initial guess (usually zero), and looking for npv to change its value.
    // We are interested in a root closest to the initial guess, so we choose a conservative (close to 1.0) factor
    // to expand the search area on each iteration.
    let bounds_search_expansion_factor = 1.1;

    match find_root(guess, f_npv, bounds_search_expansion_factor) {
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

    #[test]
    fn irr_finds_closest_root_to_zero() {
        let cf = [10., 20., -10.];
        let guess = Some(0.);
        assert_eq!(irr(&cf, guess).unwrap(), -0.5857864377789364); // there's also a solution of -3.414213531256609 which we don't want
    }
}
