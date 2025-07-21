#![allow(clippy::missing_panics_doc, clippy::unused_self)]

use std::{
    borrow::Borrow,
    fmt,
    ops::{self, Add, Bound, Sub},
};

use crate::{ContinuousRange, RangesRelation};

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Hash, PartialEq)]
pub enum Range<Idx> {
    Continuous(ContinuousRange<Idx>),
    Composite(Vec<ContinuousRange<Idx>>),
}

impl<Idx> From<ContinuousRange<Idx>> for Range<Idx> {
    fn from(r: ContinuousRange<Idx>) -> Self {
        Self::Continuous(r)
    }
}

impl<Idx: PartialOrd + Clone> Range<Idx> {
    /// A range containing no value
    ///
    /// `[]`
    #[must_use]
    pub fn empty() -> Self {
        Range::Continuous(ContinuousRange::empty())
    }

    /// A range containing a single value
    ///
    /// `value`
    #[must_use]
    pub fn single(value: Idx) -> Range<Idx> {
        Range::Continuous(ContinuousRange::single(value))
    }

    /// A range between `start` (inclusive) and `end` (inclusive)
    ///
    /// `[start..end]`
    #[must_use]
    pub fn continuous(start: Idx, end: Idx) -> Self
    where
        Idx: PartialOrd,
    {
        Range::Continuous(ContinuousRange::inclusive(start, end))
    }

    /// A range between `start` (exclusive) and `end` (exclusive)
    ///
    /// `(start..end)`
    #[must_use]
    pub fn continuous_exclusive(start: Idx, end: Idx) -> Self
    where
        Idx: PartialOrd,
    {
        Range::Continuous(ContinuousRange::exclusive(start, end))
    }

    /// A range between `start` (exclusive) and `end` (inclusive)
    ///
    /// `(start..end]`
    #[must_use]
    pub fn continuous_start_exclusive(start: Idx, end: Idx) -> Range<Idx>
    where
        Idx: PartialOrd,
    {
        Range::Continuous(ContinuousRange::start_exclusive(start, end))
    }

    /// A range between `start` (inclusive) and `end` (exclusive)
    ///
    /// `[start..end)`
    #[must_use]
    pub fn continuous_end_exclusive(start: Idx, end: Idx) -> Range<Idx>
    where
        Idx: PartialOrd,
    {
        Range::Continuous(ContinuousRange::end_exclusive(start, end))
    }

    /// A range starting from `start` (inclusive)
    ///
    /// `[start..)`
    #[must_use]
    pub fn from(start: Idx) -> Self {
        Self::Continuous(ContinuousRange::from(start))
    }

    /// A range starting from `start` (exclusive)
    ///
    /// `(start..)`
    #[must_use]
    pub fn from_exclusive(start: Idx) -> Self {
        Self::Continuous(ContinuousRange::from_exclusive(start))
    }

    /// A range ending with `end` (inclusive)
    ///
    /// `(..end]`
    #[must_use]
    pub fn to(end: Idx) -> Self {
        Self::Continuous(ContinuousRange::to(end))
    }

    /// A range ending with `end` (exclusive)
    ///
    /// `(..end)`
    #[must_use]
    pub fn to_exclusive(end: Idx) -> Self {
        Self::Continuous(ContinuousRange::to_exclusive(end))
    }

    /// A range containing all values
    #[must_use]
    pub fn full() -> Range<Idx> {
        Self::Continuous(ContinuousRange::full())
    }

    #[must_use]
    pub fn composite(items: impl IntoIterator<Item = Range<Idx>>) -> Range<Idx>
    where
        Idx: PartialOrd,
    {
        let mut iter = items.into_iter();

        match iter.size_hint() {
            (_, Some(0)) => Self::empty(),
            (1, Some(1)) => iter.next().unwrap(),
            _ => {
                let mut new_items = vec![];

                for item in iter {
                    if item.is_empty() {
                        continue;
                    }

                    if item.is_full() {
                        return item;
                    }

                    match item {
                        Self::Composite(v) => {
                            new_items.extend(v);
                        }
                        Self::Continuous(r) => new_items.push(r),
                    }
                }

                match new_items.len() {
                    0 => Self::empty(),
                    1 => Self::Continuous(new_items.into_iter().next().unwrap()),
                    _ => Self::Composite(new_items),
                }
            }
        }
    }

    #[must_use]
    pub fn range_bounds(&self) -> Option<(Bound<&Idx>, Bound<&Idx>)> {
        match self {
            Self::Continuous(r) => r.range_bounds(),
            Self::Composite(_) => None, // TODO: custom implementation of bounds
        }
    }

    #[must_use]
    pub fn contains(&self, value: impl Borrow<Idx>) -> bool
    where
        Idx: PartialOrd,
    {
        match self {
            Self::Continuous(r) => r.contains(value),
            Self::Composite(r) => {
                let value = value.borrow();
                r.iter().any(|x| x.contains(value))
            }
        }
    }

    #[must_use]
    pub fn union(&self, other: &Range<Idx>) -> Range<Idx>
    where
        Idx: PartialOrd,
    {
        // TODO: Quite a few cases can be optimized, specialized, ...
        // TODO: Also maybe the ranges should be kept sorted in composite
        match (self, other) {
            (Self::Continuous(ContinuousRange::Empty), r)
            | (r, Self::Continuous(ContinuousRange::Empty)) => r.clone(),
            (Self::Continuous(ContinuousRange::Full), _)
            | (_, Self::Continuous(ContinuousRange::Full)) => Self::full(),
            (r1, r2) => Range::composite(vec![r1.clone(), r2.clone()]),
        }
    }

    #[must_use]
    pub fn intersection(self, _other: &Range<Idx>) -> Range<Idx> {
        todo!()
    }

    #[must_use]
    pub fn difference(self, _other: &Range<Idx>) -> Range<Idx> {
        todo!()
    }

    #[must_use]
    pub fn overlaps(self, _other: &Range<Idx>) -> bool {
        todo!()
    }

    #[must_use]
    /// Compare the bounds of two ranges
    pub fn compare_bounds(&self, _other: &Range<Idx>) -> RangesRelation {
        // Inspired from "Maintaining Knowledge about Temporal Intervals"
        todo!()
    }

    pub fn simplify_mut(&mut self)
    where
        Idx: PartialOrd,
    {
        match self {
            Self::Continuous(r) => r.simplify_mut(),
            Self::Composite(v) => {
                // TODO: Handle much more cases
                // - Empty ranges
                // - Full ranges
                // - Ranges with only one element
                // - Merge overlapping ranges
                // - ...
                for item in &mut *v {
                    item.simplify_mut();
                }
            }
        }
    }

    #[must_use]
    pub fn simplify(&self) -> Self
    where
        Idx: PartialOrd + Clone,
    {
        let mut clone = (*self).clone();
        clone.simplify_mut();
        clone
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Continuous(r) => r.is_empty(),
            Self::Composite(v) => v.iter().all(ContinuousRange::is_empty),
        }
    }

    #[must_use]
    pub fn is_full(&self) -> bool
    where
        Idx: PartialOrd + Clone,
    {
        // We simplify to handle case that are complex but represent the full
        // range when simplified like (-inf, 0]; [0, +Inf)
        match self.simplify() {
            Self::Continuous(r) => r.is_full(),
            Self::Composite(v) => v.iter().any(ContinuousRange::is_full),
        }
    }
}

impl<Idx: PartialOrd + Clone> Add<&Range<Idx>> for Range<Idx> {
    type Output = Range<Idx>;

    fn add(self, other: &Range<Idx>) -> Range<Idx> {
        self.union(other)
    }
}

impl<Idx: PartialOrd + Clone> Sub<Range<Idx>> for Range<Idx> {
    type Output = Range<Idx>;

    fn sub(self, other: Range<Idx>) -> Range<Idx> {
        self.difference(&other)
    }
}

impl<Idx: PartialOrd + Clone> From<ops::RangeFull> for Range<Idx> {
    fn from(_: ops::RangeFull) -> Self {
        Self::full()
    }
}

impl<Idx: PartialOrd + Clone> From<ops::Range<Idx>> for Range<Idx> {
    fn from(r: ops::Range<Idx>) -> Self {
        Self::continuous_end_exclusive(r.start, r.end)
    }
}

impl<Idx: PartialOrd + Clone> From<ops::RangeInclusive<Idx>> for Range<Idx> {
    fn from(r: ops::RangeInclusive<Idx>) -> Self {
        let (start, end) = r.into_inner();
        Self::continuous(start, end)
    }
}

impl<Idx: PartialOrd + Clone> From<ops::RangeFrom<Idx>> for Range<Idx> {
    fn from(r: ops::RangeFrom<Idx>) -> Self {
        Self::from(r.start)
    }
}

impl<Idx: PartialOrd + Clone> From<ops::RangeToInclusive<Idx>> for Range<Idx> {
    fn from(r: ops::RangeToInclusive<Idx>) -> Self {
        Self::to(r.end)
    }
}

impl<Idx: PartialOrd + Clone> From<ops::RangeTo<Idx>> for Range<Idx> {
    fn from(r: ops::RangeTo<Idx>) -> Self {
        Self::to_exclusive(r.end)
    }
}

impl<Idx: fmt::Debug> fmt::Debug for Range<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Range::Continuous(r) => {
                r.fmt(fmt)?;
            }
            Range::Composite(r) => {
                write!(fmt, "{{")?;
                let mut first = true;
                for child in r {
                    if first {
                        first = false;
                    } else {
                        write!(fmt, "; ")?;
                    }

                    fmt::Debug::fmt(child, fmt)?;
                }
                write!(fmt, "}}")?;
            }
        }
        Ok(())
    }
}

impl<Idx> Default for Range<Idx> {
    fn default() -> Self {
        Self::Continuous(ContinuousRange::Empty)
    }
}
