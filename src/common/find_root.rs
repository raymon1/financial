use crate::common::PRECISION;

const NEWTON_MAX_ITERATION: u32 = 20;
const BISECTION_MAX_ITERATION: u32 = 2000;
const INITIAL_GUESS: f64 = 0.;

pub fn find_root<F>(x: Option<f64>, func: F) -> Option<f64>
where
    F: Fn(f64) -> f64,
{
    let x = match x {
        Some(num) => num,
        None => INITIAL_GUESS,
    };
    let f = |x| func(x);
    let newton_val = newton(x, f);
    let same_sign = |x: f64, y: f64| {
        x.is_sign_positive() && y.is_sign_positive() || x.is_sign_negative() && y.is_sign_negative()
    };

    if newton_val.is_some() && same_sign(newton_val.unwrap(), x) {
        newton_val
    } else {
        if let Some(b_pos) = find_bounds(x, Bounds::new_positive(), f) {
            bisection(b_pos, f)
        } else if let Some(b_neg) = find_bounds(x, Bounds::new_negative(), f) {
            bisection(b_neg, f)
        }
        else {
            None
        }
    }
}

fn newton<F>(x: f64, f: F) -> Option<f64>
where
    F: Fn(f64) -> f64,
{
    let mut x = x;
    let df = |x: f64| (f(x + PRECISION) - f(x - PRECISION)) / (2. * PRECISION);

    for _ in 1..NEWTON_MAX_ITERATION {
        let fx = f(x);
        let dfx = df(x);

        let new_x = x - fx / dfx;

        if (new_x - x).abs() <= PRECISION || fx.abs() <= PRECISION{
            return Some(new_x);
        }

        x = new_x;
    }

    None
}

fn bisection<F>(bounds: Bounds, f: F) -> Option<f64>
where
    F: Fn(f64) -> f64,
{
    let mut a = bounds.lower;
    let mut b = bounds.upper;
    for _ in 1..BISECTION_MAX_ITERATION {
        let fa = f(a);
        if fa.abs() < PRECISION {
            return Some(a);
        } else {
            let fb = f(b);
            if fb.abs() < PRECISION {
                return Some(b);
            } else {
                if fa * fb > 0. {
                    return None;
                }

                let mid = a + (b - a) / 2.;
                let fmid = f(mid);

                if fmid.abs() < PRECISION {
                    return Some(mid);
                } else {
                    let fafmid = fa * fmid;
                    if fafmid < 0. {
                        b = mid;
                    } else if fafmid > 0. {
                        a = mid;
                    } else {
                        panic!("it should never get here");
                    }
                }
            }
        }
    }

    None
}

fn find_bounds<F>(x: f64, bounds: Bounds, f: F) -> Option<Bounds>
where
    F: Fn(f64) -> f64,
{
    let shift = 0.01;
    let factor = 1.6;
    let adjust_to_min = |val| {
        if val <= bounds.lower {
            bounds.lower + PRECISION
        } else {
            val
        }
    };
    let adjust_to_max = |val| {
        if val >= bounds.upper {
            bounds.upper - PRECISION
        } else {
            val
        }
    };

    let mut low = adjust_to_min(x - shift);
    let mut upp = adjust_to_max(x + shift);
    for _ in 1..60 {
        let lower = adjust_to_min(low);
        let upper = adjust_to_max(upp);
        let product = f(lower) * f(upper);
        if product <= 0. {
            return Some(Bounds::new_from_range(lower, upper));
        } else {
            low = lower + factor * (lower - upper);
            upp = upper + factor * (upper - lower);
            continue;
        }
    }

    None
}

struct Bounds {
    lower: f64,
    upper: f64,
}

impl Bounds {
    fn new_positive() -> Bounds {
        Bounds {
            lower: 0.,
            upper: f64::MAX,
        }
    }

    fn new_negative() -> Bounds {
        Bounds {
            lower: f64::MIN,
            upper: 0.,
        }
    }

    fn new_from_range(lower: f64, upper: f64) -> Bounds {
        if lower > upper {
            panic!("lower cannot be greater than upper bounds")
        }
        Bounds { lower, upper }
    }
}
