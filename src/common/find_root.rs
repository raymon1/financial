use crate::common::PRECISION;

const NEWTON_MAX_ITERATION: u32 = 60;
const INITIAL_GUESS: f64 = 0.;

// TODO: add bisection method
pub fn find_root<F>(x: Option<f64>, func: F) -> Option<f64>
where
    F: Fn(f64) -> f64,
{
    newton(x, func)
}

fn newton<F>(x: Option<f64>, f: F) -> Option<f64>
where
    F: Fn(f64) -> f64,
{
    let mut x = match x {
        None => INITIAL_GUESS,
        Some(guess) => guess,
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
