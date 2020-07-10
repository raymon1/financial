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

    // trimming zeros from both ends
    let begin = values.iter()
        .position(not_zero)
        .unwrap_or(0);

    let end = values.len() - values.iter()
        .rev()
        .position(not_zero)    
        .unwrap_or(1);

    &values[begin..end]
}
