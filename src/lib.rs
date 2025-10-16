#![cfg_attr(feature = "nightly", feature(f16, f128))]
#![cfg_attr(any(docsrs, feature = "nightly"), feature(doc_cfg))]
#![no_std]

//! A simple crate to limit values using range syntax.
//!
//! Specifically, `start..`, `..=end`, `start..=end`, and `..` are supported.
//!
//! There is a separate trait for floating point types, due to coherence rules.

mod ord;
mod float;

pub use ord::{Limit, LimitBounds};
pub use float::{LimitFloat, LimitFloatBounds};
