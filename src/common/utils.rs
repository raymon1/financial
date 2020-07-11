pub const ACCURACY: f64= 1e-7;
const NEWTON_MAX_ITERATION: u32 = 60;
const INITIAL_GUESS: f64 = 0.;

pub fn powers(base: &f64, n: &usize, start_from_zero: bool) -> Vec<f64>{
    let start = if start_from_zero { 0 } else { 1 }; 
    let p0 = if start_from_zero { 1. } else { *base };
    
    (start..*n)
    .fold(vec![p0], |mut pows, _| {
        pows.push(pows.last().unwrap() * (base)); 
        pows
    })
}

pub fn trim_zeros(values: &[f64]) -> &[f64] {
    let not_zero = |x: &f64| *x != 0.;

    let begin = values.iter()
        .position(not_zero)
        .unwrap_or(0);

    let end = values.len() - values.iter()
        .rev()
        .position(not_zero)    
        .unwrap_or(1);

    &values[begin..end]
}

pub fn newton(values: &[f64], guess: &Option<f64>) -> Option<f64> {
    let mut guess = match guess {
        None => INITIAL_GUESS,
        Some(g) => *g
    };

    for _ in 1..NEWTON_MAX_ITERATION {
        let powers = powers(&(1. + guess), &values.len(), true);

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

        if (new_guess - guess).abs() <= ACCURACY {
            return Some(new_guess);
        }

        guess = new_guess;
    }

    Some(guess)
}

// TODO: add bisection method
pub fn find_root(values: &[f64], guess: &Option<f64>) -> Option<f64> {
    newton(&values, &guess)
}

use chrono::{DateTime, offset::TimeZone};

#[inline]
pub fn days_to<T: TimeZone>(d0: DateTime<T>, d1: DateTime<T>) -> f64 {
    d1.signed_duration_since(d0).num_days() as f64
}