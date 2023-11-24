//! Prayertimes calculator for rust based on [Praytimes.org](http://praytimes.org)
//!
//! for information about the calculation see [calculation](http://praytimes.org/calculation)
//!
//! see [`Calculator`] for calculation
mod internal_calculator;
pub mod methods;
pub mod types;
mod utils;

mod calculator;

pub use calculator::Calculator;
pub mod iter;
