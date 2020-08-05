//! # Financial
//!
//! `Financial` is a collection of finance calculations mimicking some of Excel Financial Functions interface.
//!
//! ## What makes this crate different
//!
//! It supports both periodic and scheduled computation for [IRR](https://en.wikipedia.org/wiki/Internal_rate_of_return) and [NPV](https://en.wikipedia.org/wiki/Net_present_value).
//!
//! IRR and NPV functions are faster since powers are pre-computed iteratively instead of using power function multiple times. Take this with a grain of salt since no benches tests are offered at the moment.
//!
//! ## Supported Functions
//!
//! - FV(Rate, Nper, Pmt, Pv, Pmt_is_due)
//! - PV(Rate, Nper, Pmt, Fv, Pmt_is_due)
//! - NPV(Rate, values)
//! - XNPV(Rate), values, dates)
//! - IRR(values)
//! - XIRR(values, dates)
//! - MIRR(values, finance_rate, reinvest_rate)
//!
//! ## Future Work
//!
//! - Add More Functions (NPER, PMT, Rate, effect)
//!
//! ## Testing
//!
//! - This crate has over 180 test case, most of them are compared to Excel outputs.
//! - XIRR is not compared against Excel, since Excel XIRR doesn't always converge to the correct answer and often produce the wrong answer of 0.000000002980.
//! Instead XIRR are tested by using the XIRR to produce a zero XNPV value.
//! - Note that the precision used for equality of floating points is 1e-7
//!
//! ## Contribution
//!
//! - Using the crate and providing feedback or pointing out any issues.
//! - Adding more test cases is encouraged.
//! - Any contribution that serves the crate is welcome.
//!
//! [Github](https://github.com/raymon1/financial)
//!

mod common;
mod periodic_cashflow;
mod scheduled_cashflow;

pub use crate::periodic_cashflow::fv::fv;
pub use crate::periodic_cashflow::irr::irr;
pub use crate::periodic_cashflow::mirr::mirr;
pub use crate::periodic_cashflow::npv::npv;
pub use crate::periodic_cashflow::pv::pv;
pub use crate::scheduled_cashflow::xirr::xirr;
pub use crate::scheduled_cashflow::xnpv::xnpv;
