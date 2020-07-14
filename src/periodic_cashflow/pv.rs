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
pub fn pv(
    rate: &f64,
    nper: &f64,
    pmt: &Option<f64>,
    fv: &Option<f64>,
    pmt_at_begining: &Option<bool>,
) -> f64 {
    let pmt = pmt.unwrap_or_else(|| 0.);
    let fv = fv.unwrap_or_else(|| 0.);

    if *rate == 0.0 {
        return -(fv + pmt * nper);
    } else {
        let pmt_at_begining = if pmt_at_begining.unwrap_or_else(|| false) {
            1.
        } else {
            0.
        };
        let temp = f64::powf(1. + rate, *nper);
        let factor = (1. + rate * pmt_at_begining) * (temp - 1.) / rate;
        -(fv + pmt * factor) / temp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pv_works_when_pmt_at_end_of_period() {
        assert_eq!(
            pv(&0.1, &5.0, &Some(100.0), &Some(1000.0), &Some(false)),
            -1000.0000000000001
        );
    }

    #[test]
    fn pv_works_when_pmt_at_beginning() {
        assert_eq!(
            pv(&0.1, &5.0, &Some(100.0), &Some(1000.0), &Some(true)),
            -1037.90786769408449
        );
    }

    #[test]
    fn pv_works_with_pmt_only() {
        assert_eq!(
            pv(&0.1, &5.0, &Some(100.0), &None, &Some(false)),
            -379.07867694084507
        );
    }

    #[test]
    fn pv_works_with_zero_rate() {
        assert_eq!(pv(&0.0, &5.0, &Some(100.0), &Some(1000.0), &None), -1500.0);
    }
}
