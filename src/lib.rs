#![allow(
    clippy::bool_assert_comparison,
    clippy::match_same_arms,
    clippy::module_name_repetitions,
)]

mod continuous;
mod range;
mod relation;
mod bounds;

pub use continuous::ContinuousRange;
pub use range::Range;
pub use relation::RangesRelation;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod continuous_tests;
