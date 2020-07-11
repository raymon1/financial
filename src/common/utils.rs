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

use chrono::{DateTime, offset::TimeZone};

#[inline]
pub fn days_to<T: TimeZone>(d0: DateTime<T>, d1: DateTime<T>) -> f64 {
    d1.signed_duration_since(d0).num_days() as f64
}

// FIXME: needs to move
pub fn validate_cashflow_values(values: &[f64]) -> Result<(), &'static str> {
    let len = values.len();
    let zeros = values.iter().filter(|x| **x == 0.).count();
    let negatives = values.iter().filter(|x| x.is_sign_negative()).count();

    if len < 2 || zeros + negatives == len {
        return Err("cashflow must contain more than one value, and include positive and negative values");
    }

    Ok(())
}
