use crate::common::PRECISION;

const NEWTON_MAX_ITERATION: u32 = 60;
const INITIAL_GUESS: f64 = 0.;

// TODO: add bisection method
pub fn find_root<F> (x: &Option<f64>, func: F) -> Option<f64>
where F: Fn(f64) -> f64 {
    newton(&x, func)
}

fn newton<F> (x: &Option<f64>, f: F) -> Option<f64> 
where F: Fn(f64) -> f64 {
    let mut x = match x {
        None => INITIAL_GUESS,
        Some(guess) => *guess
    };

    let df = |x: f64| (f(x + PRECISION) - f(x - PRECISION)) / (2. * PRECISION);
    
    for _ in 1..NEWTON_MAX_ITERATION {
        let fx = f(x);
        let dfx = df(x);
        
        let new_x = x - fx / dfx;
                
        if (new_x - x).abs() <= PRECISION {
            return Some(new_x);
        }

        x = new_x;
    }

    None
}

fn newton2(values: &[f64], guess: &Option<f64>) -> Option<f64> {
    let mut guess = match guess {
        None => INITIAL_GUESS,
        Some(g) => *g
    };

    for _ in 1..NEWTON_MAX_ITERATION {
        let powers = crate::common::utils::powers(&(1. + guess), &values.len(), true);

        let f_value : f64 = powers.iter()
            .zip(values.iter())
            .map(|(p, v)| v / p)
            .sum();

        let f_derivative : f64 = 
            (0..values.len() - 1)
            .zip(powers.iter().skip(1))
            .zip(values.iter())
            .map(|((i, p), v)| - f64::from(i as i32) * v / p)
            .sum();
        
        let new_guess = guess - f_value / f_derivative;

        if (new_guess - guess).abs() <= PRECISION {
            return Some(new_guess);
        }

        guess = new_guess;
    }

    Some(guess)
}