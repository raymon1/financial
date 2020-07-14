/// Calculates the future value of an investment based on a constant interest rate.
/// You can use FV with either periodic, constant payments, or a single lump sum payment.
///
/// # Examples
///
/// ```
/// let fv = financial::fv(&0.1, &5.0, &Some(100.0), &Some(1000.0), &Some(false));
/// assert_eq!(fv, -2221.020000000001);
/// ```
pub fn fv(
    rate: &f64,
    nper: &f64,
    pmt: &Option<f64>,
    pv: &Option<f64>,
    pmt_at_begining: &Option<bool>,
) -> f64 {
    let factor = |r| f64::powf(1.0 + r, *nper);

    let pmt = pmt.unwrap_or_else(|| 0.0);
    let pv = pv.unwrap_or_else(|| 0.0);

    if *rate == 0.0 {
        -(pv + pmt * nper)
    } else {
        let factor = factor(rate);
        let pmt_at_begining = if pmt_at_begining.unwrap_or_else(|| false) {
            1.0
        } else {
            0.0
        };

        -pv * factor - pmt * (1.0 + rate * pmt_at_begining) / rate * (factor - 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fv_works_when_pmt_at_end_of_period() {
        assert_eq!(
            fv(&0.1, &5.0, &Some(100.0), &Some(1000.0), &Some(false)),
            -2221.020000000001
        );
    }

    #[test]
    fn fv_works_when_pmt_at_beginning() {
        assert_eq!(
            fv(&0.1, &5.0, &Some(100.0), &Some(1000.0), &Some(true)),
            -2282.071000000001
        );
    }

    #[test]
    fn fv_works_with_pmt_only() {
        assert_eq!(
            fv(&0.1, &5.0, &Some(100.0), &None, &Some(false)),
            -610.5100000000006
        );
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

        let results: Vec<f64> = rates
            .iter()
            .zip(pvs.iter())
            .zip(pmt_at_begining.iter())
            .map(|(rpv, a)| fv(&rpv.0, &nper, &pmt, &rpv.1, &a))
            .collect();

        assert_eq!(
            results,
            [
                -2221.020000000001,
                -2282.071000000001,
                -610.5100000000006,
                -1500.0
            ]
        );
    }
}
