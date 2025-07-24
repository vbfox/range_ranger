#![allow(
    clippy::bool_assert_comparison,
    clippy::match_same_arms,
    clippy::module_name_repetitions
)]

mod bounds;
mod continuous;
mod range;
mod range_builder;
mod relation;
mod simplify;

pub use continuous::ContinuousRange;
pub use range::Range;
pub use range_builder::RangeBuilder;
pub use relation::RangesRelation;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod continuous_tests;
