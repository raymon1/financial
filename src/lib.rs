//! # Financial
//!
//! `Financial` is a collection of finance calculations memicking some of Excel Financial Functions interface.

mod common;
mod periodic_cashflow;
mod scheduled_cashflow;

pub use crate::periodic_cashflow::irr::irr;
pub use crate::periodic_cashflow::fv::fv;
pub use crate::periodic_cashflow::pv::pv;
pub use crate::periodic_cashflow::npv::npv;
pub use crate::scheduled_cashflow::xnpv::xnpv;
pub use crate::scheduled_cashflow::xirr::xirr;
