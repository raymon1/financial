//! # Financial
//!
//! `Financial` is a collection of finance calculations mimicking some of Excel Financial Functions interface.
//! ## What is different than other financial Rust crates?
//!
//! It supports both periodic and scheduled computation for [IRR](https://en.wikipedia.org/wiki/Internal_rate_of_return) and [NPV](https://en.wikipedia.org/wiki/Net_present_value).
//!
//! ## Future Work
//!
//! - Add More Functions
//! - Add bench tests
//!
//! [Github](https://github.com/raymon1/financial)
//!

mod common;
mod periodic_cashflow;
mod scheduled_cashflow;

pub use crate::periodic_cashflow::fv::fv;
pub use crate::periodic_cashflow::irr::irr;
pub use crate::periodic_cashflow::npv::npv;
pub use crate::periodic_cashflow::pv::pv;
pub use crate::scheduled_cashflow::xirr::xirr;
pub use crate::scheduled_cashflow::xnpv::xnpv;

use std::slice;

#[no_mangle]
pub extern "C" fn npv2(rate: f64, values: *const f64) -> f64 {
    let numbers = unsafe { slice::from_raw_parts(values, 4 as usize) };

    npv(rate, numbers)
}

#[no_mangle]
pub extern "C" fn add_numbers(number1: i32, number2: i32) -> i32 {
    println!("Hello from rust!");
    number1 + number2
}
