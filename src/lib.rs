//! # Financial
//!
//! `Financial` is a collection of finance calculations memicking some of Excel Financial Functions interface.

mod common;
mod periodic_cashflow;

pub use crate::periodic_cashflow::irr::irr;
pub use crate::periodic_cashflow::fv::fv;
pub use crate::periodic_cashflow::pv::pv;
pub use crate::periodic_cashflow::npv::npv;
