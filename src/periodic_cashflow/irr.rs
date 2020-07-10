use crate::common::utils;

const IRR_MAX_ITERATION: u32 = 100;
const IRR_ACCURACY: f64= 1e-7;
const IRR_INITIAL_GUESS: f64 = 0.;

/// calculates the internal rate of return for a series of cash flows occurring at regular interval represented by the numbers in values.
pub fn irr(values: &[f64], guess: &Option<f64>) -> Result<f64, &'static str> {
    let values = utils::trim_zeros(&values);

    let len = values.len();
    let zeros = values.iter().filter(|x| **x == 0.).count();
    let negatives = values.iter().filter(|x| x.is_sign_negative()).count();

    if len < 2 || zeros + negatives == len {
        return Err("cashflow must contain more than one value, and include positive and negative values");
    }
    let mut guess = match guess {
        None => IRR_INITIAL_GUESS,
        Some(g) => *g
    };

    for _ in 1..IRR_MAX_ITERATION {
        let powers = utils::powers(&(1. + guess), &len, true);

        let f_value : f64 = powers.iter()
            .zip(values.iter())
            .map(|(p, v)| v / p)
            .sum();

        let f_derivative : f64 = 
            (0..len - 1)
            .zip(powers.iter().skip(1))
            .zip(values.iter())
            .map(|((i, p), v)| - f64::from(i as i32) * v / p)
            .sum();
        
        let new_guess = guess - f_value / f_derivative;

        if new_guess.abs() <= IRR_ACCURACY {
            return Ok(new_guess);
        }

        guess = new_guess;
    }

    // Err("could't find irr for the values provided")
    Ok(guess)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn irr_works() {
        let cf = [-500., 100., 100., 100., 100.];
        let guess = Some(0.);
        let accuracy = (irr(&cf, &guess).unwrap() - -0.08364541746615000000000000000000).abs();
        assert!(accuracy <= IRR_ACCURACY, format!("exceeded IRR accuracy threshold, {}", accuracy));
    }

    #[test]
    fn irr_works_2() {
        let cf = [-500., 100., 100., 100., 100., 100.];
        let guess = Some(0.);
        assert_eq!(irr(&cf, &guess).unwrap(), 0.0);
    }

}
