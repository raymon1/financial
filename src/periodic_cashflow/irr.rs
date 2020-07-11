use crate::common::utils;

/// calculates the internal rate of return for a series of cash flows occurring at regular interval represented by the numbers in values.
pub fn irr(values: &[f64], guess: &Option<f64>) -> Result<f64, &'static str> {
    let values = utils::trim_zeros(&values);

    let len = values.len();
    let zeros = values.iter().filter(|x| **x == 0.).count();
    let negatives = values.iter().filter(|x| x.is_sign_negative()).count();

    if len < 2 || zeros + negatives == len {
        return Err("cashflow must contain more than one value, and include positive and negative values");
    }
    
    match utils::find_root(&values, &guess) {
        Some(ans) => Ok(ans),
        None => Err("could't find irr for the values provided")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn irr_works() {
        let cf = [-500., 100., 100., 100., 100.];
        let guess = Some(0.);
        let accuracy = (irr(&cf, &guess).unwrap() - -0.08364541746615000000000000000000).abs();
        assert!(accuracy <= utils::ACCURACY, format!("exceeded IRR accuracy threshold, {}", accuracy));
    }

    #[test]
    fn irr_works_2() {
        let cf = [-500., 100., 100., 100., 100., 100.];
        let guess = Some(0.);
        assert_eq!(irr(&cf, &guess).unwrap(), 0.0);
    }

}
