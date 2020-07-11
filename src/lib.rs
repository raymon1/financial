//! # Financial
//!
//! `Financial` is a collection of finance calculations memicking some of Excel Financial Functions interface.
//! ## What is different than other financial Rust cargos?
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

pub use crate::periodic_cashflow::irr::irr;
pub use crate::periodic_cashflow::fv::fv;
pub use crate::periodic_cashflow::pv::pv;
pub use crate::periodic_cashflow::npv::npv;
pub use crate::scheduled_cashflow::xnpv::xnpv;
pub use crate::scheduled_cashflow::xirr::xirr;
