use std::{
    borrow::Borrow,
    cmp::Ordering,
    convert::TryFrom,
    fmt,
    ops::{self, Bound},
};

use crate::RangesRelation;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Hash, PartialEq)]
pub enum ContinuousRange<Idx> {
    /// A range containing no value
    ///
    /// `[]`
    Empty,

    /// A range containing a single value
    ///
    /// `value`
    Single(Idx),

    /// A range between `start` (inclusive) and `end` (inclusive)
    ///
    /// `[start..end]`
    Inclusive(Idx, Idx),

    /// A range between `start` (exclusive) and `end` (exclusive)
    ///
    /// `(start..end)`
    Exclusive(Idx, Idx),

    /// A range between `start` (exclusive) and `end` (inclusive)
    ///
    /// `(start..end]`
    StartExclusive(Idx, Idx),

    /// A range between `start` (inclusive) and `end` (exclusive)
    ///
    /// `[start..end)`
    EndExclusive(Idx, Idx),

    /// A range starting from `start` (inclusive)
    ///
    /// `[start..)`
    From(Idx),

    /// A range starting from `start` (exclusive)
    ///
    /// `(start..)`
    FromExclusive(Idx),

    /// A range ending with `end` (inclusive)
    ///
    /// `(..end]`
    To(Idx),

    /// A range ending with `end` (exclusive)
    ///
    /// `(..end)`
    ToExclusive(Idx),

    /// A range containing all values
    Full,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum BoundSide {
    Start,
    End,
}

/// An endpoint of a continuous range
pub struct ContinuousBound<T>(Option<Bound<T>>);

impl<T> ContinuousBound<T> {
    /// Create a new bound
    pub fn new(bound: Option<Bound<T>>) -> Self {
        Self(bound)
    }

    /// Get the bound
    pub fn bound(&self) -> Option<Bound<T>>
    where
        T: Copy,
    {
        self.0
    }

    pub fn into_inner(self) -> Option<Bound<T>> {
        self.0
    }

    pub fn value(&self) -> Option<T>
    where
        T: Copy,
    {
        match &self.0 {
            Some(Bound::Included(x)) | Some(Bound::Excluded(x)) => Some(*x),
            _ => None,
        }
    }

    pub fn unwrap(self) -> Bound<T> {
        match self.0 {
            Some(val) => val,
            None => panic!("called `ContinuousBound::unwrap()` on a `None` value"),
        }
    }

    pub fn expect(self, msg: &str) -> Bound<T> {
        match self.0 {
            Some(val) => val,
            None => panic!("{}", msg),
        }
    }
}

impl<T> From<ContinuousBound<T>> for Option<Bound<T>> {
    fn from(bound: ContinuousBound<T>) -> Self {
        bound.0
    }
}

impl<T> From<Option<Bound<T>>> for ContinuousBound<T> {
    fn from(bound: Option<Bound<T>>) -> Self {
        Self(bound)
    }
}

impl<T> From<Bound<T>> for ContinuousBound<T> {
    fn from(bound: Bound<T>) -> Self {
        Self(Some(bound))
    }
}

impl<T> TryFrom<ContinuousBound<T>> for Bound<T> {
    type Error = ();

    fn try_from(bound: ContinuousBound<T>) -> Result<Self, Self::Error> {
        match bound.0 {
            Some(val) => Ok(val),
            None => Err(()),
        }
    }
}

fn partial_cmp_bounds<Idx: PartialOrd>(
    this: &Bound<&Idx>,
    this_side: BoundSide,
    other: &Bound<&Idx>,
    other_side: BoundSide,
) -> Option<Ordering> {
    match this {
        Bound::Included(this_value) => match other {
            Bound::Included(other_value) => this_value.partial_cmp(other_value),
            Bound::Excluded(other_value) => match this_value.partial_cmp(other_value) {
                Some(Ordering::Equal) => match (this_side, other_side) {
                    (BoundSide::Start, BoundSide::Start) => Some(Ordering::Less),
                    (BoundSide::End, BoundSide::End) => Some(Ordering::Greater),
                    (BoundSide::Start, BoundSide::End) => Some(Ordering::Greater),
                    (BoundSide::End, BoundSide::Start) => Some(Ordering::Less),
                },
                other => other,
            },
            Bound::Unbounded => match other_side {
                BoundSide::Start => Some(Ordering::Greater), // -Inf
                BoundSide::End => Some(Ordering::Less),      // +Inf
            },
        },
        Bound::Excluded(this_value) => match other {
            Bound::Included(other_value) => match this_value.partial_cmp(other_value) {
                Some(Ordering::Equal) => match (this_side, other_side) {
                    (BoundSide::Start, BoundSide::Start) => Some(Ordering::Greater),
                    (BoundSide::End, BoundSide::End) => Some(Ordering::Less),
                    (BoundSide::Start, BoundSide::End) => Some(Ordering::Greater),
                    (BoundSide::End, BoundSide::Start) => Some(Ordering::Less),
                },
                other => other,
            },
            Bound::Excluded(other_value) => match this_value.partial_cmp(other_value) {
                Some(Ordering::Equal) => match (this_side, other_side) {
                    (BoundSide::Start, BoundSide::Start) => Some(Ordering::Equal),
                    (BoundSide::End, BoundSide::End) => Some(Ordering::Equal),
                    (BoundSide::Start, BoundSide::End) => Some(Ordering::Greater),
                    (BoundSide::End, BoundSide::Start) => Some(Ordering::Less),
                },
                other => other,
            },
            Bound::Unbounded => match other_side {
                BoundSide::Start => Some(Ordering::Greater), // -Inf
                BoundSide::End => Some(Ordering::Less),      // +Inf
            },
        },
        Bound::Unbounded => match other {
            Bound::Included(_) | Bound::Excluded(_) => match this_side {
                BoundSide::Start => Some(Ordering::Less),  // -Inf
                BoundSide::End => Some(Ordering::Greater), // +Inf
            },
            Bound::Unbounded => match (this_side, other_side) {
                (BoundSide::Start, BoundSide::Start) => Some(Ordering::Equal),
                (BoundSide::End, BoundSide::End) => Some(Ordering::Equal),
                (BoundSide::Start, BoundSide::End) => Some(Ordering::Less),
                (BoundSide::End, BoundSide::Start) => Some(Ordering::Greater),
            },
        },
    }
}

/// Reverse a bound between [`Bound::Included`] and [`Bound::Excluded`].
///
/// [`Bound::Unbounded`] is kept as-is.
fn reverse_bound<Idx>(bound: Bound<&Idx>) -> Bound<&Idx> {
    match bound {
        Bound::Included(x) => Bound::Excluded(x),
        Bound::Excluded(x) => Bound::Included(x),
        Bound::Unbounded => Bound::Unbounded,
    }
}

impl<Idx: PartialOrd + Clone> ContinuousRange<Idx> {
    /// A range containing no value
    ///
    /// `[]`
    #[must_use]
    pub fn empty() -> ContinuousRange<Idx> {
        ContinuousRange::Empty
    }

    /// A range containing a single value
    ///
    /// `value`
    #[must_use]
    pub fn single(value: Idx) -> ContinuousRange<Idx> {
        ContinuousRange::Single(value)
    }

    /// A range between `start` (inclusive) and `end` (inclusive)
    ///
    /// `[start..end]`
    #[must_use]
    pub fn inclusive(start: Idx, end: Idx) -> ContinuousRange<Idx>
    where
        Idx: PartialOrd,
    {
        if start == end {
            ContinuousRange::Single(start)
        } else if start > end {
            ContinuousRange::Empty
        } else {
            ContinuousRange::Inclusive(start, end)
        }
    }

    /// A range between `start` (exclusive) and `end` (exclusive)
    ///
    /// `(start..end)`
    #[must_use]
    pub fn exclusive(start: Idx, end: Idx) -> ContinuousRange<Idx>
    where
        Idx: PartialOrd,
    {
        if start >= end {
            ContinuousRange::Empty
        } else {
            ContinuousRange::Exclusive(start, end)
        }
    }

    /// A range between `start` (exclusive) and `end` (inclusive)
    ///
    /// `(start..end]`
    #[must_use]
    pub fn start_exclusive(start: Idx, end: Idx) -> ContinuousRange<Idx>
    where
        Idx: PartialOrd,
    {
        if start >= end {
            ContinuousRange::Empty
        } else {
            ContinuousRange::StartExclusive(start, end)
        }
    }

    /// A range between `start` (inclusive) and `end` (exclusive)
    ///
    /// `[start..end)`
    #[must_use]
    pub fn end_exclusive(start: Idx, end: Idx) -> ContinuousRange<Idx>
    where
        Idx: PartialOrd,
    {
        if start >= end {
            ContinuousRange::Empty
        } else {
            ContinuousRange::EndExclusive(start, end)
        }
    }

    /// A range starting from `start` (inclusive)
    ///
    /// `[start..)`
    #[must_use]
    pub fn from(start: Idx) -> ContinuousRange<Idx> {
        ContinuousRange::From(start)
    }

    /// A range starting from `start` (exclusive)
    ///
    /// `(start..)`
    #[must_use]
    pub fn from_exclusive(start: Idx) -> ContinuousRange<Idx> {
        ContinuousRange::FromExclusive(start)
    }

    /// A range ending with `end` (inclusive)
    ///
    /// `(..end]`
    #[must_use]
    pub fn to(end: Idx) -> ContinuousRange<Idx> {
        ContinuousRange::To(end)
    }

    /// A range ending with `end` (exclusive)
    ///
    /// `(..end)`
    #[must_use]
    pub fn to_exclusive(end: Idx) -> ContinuousRange<Idx> {
        ContinuousRange::ToExclusive(end)
    }

    /// A range containing all values
    #[must_use]
    pub fn full() -> ContinuousRange<Idx> {
        ContinuousRange::Full
    }

    /// Create a new range from the specified bounds
    #[must_use]
    pub fn from_bounds(bounds: (Bound<&Idx>, Bound<&Idx>)) -> Self {
        match bounds {
            (Bound::Unbounded, Bound::Unbounded) => Self::full(),
            (Bound::Included(start), Bound::Included(end)) => {
                Self::inclusive(start.clone(), end.clone())
            }
            (Bound::Included(start), Bound::Excluded(end)) => {
                Self::end_exclusive(start.clone(), end.clone())
            }
            (Bound::Included(start), Bound::Unbounded) => Self::from(start.clone()),
            (Bound::Excluded(start), Bound::Included(end)) => {
                Self::start_exclusive(start.clone(), end.clone())
            }
            (Bound::Excluded(start), Bound::Excluded(end)) => {
                Self::exclusive(start.clone(), end.clone())
            }
            (Bound::Excluded(start), Bound::Unbounded) => Self::from_exclusive(start.clone()),
            (Bound::Unbounded, Bound::Included(end)) => Self::to(end.clone()),
            (Bound::Unbounded, Bound::Excluded(end)) => Self::to_exclusive(end.clone()),
        }
    }

    /// Get the bounds of the range or [None] if empty
    #[must_use]
    pub fn range_bounds(&self) -> Option<(Bound<&Idx>, Bound<&Idx>)> {
        match self {
            Self::Empty => {
                // We can't implement RangeBounds due to this case, a possible trick would be to use a range with 2
                // exclusive bounds on the default value.
                // But as the result is a reference we would need a per-generic 'static to reference and so would
                // require something like the 'typemap' crate just for that.
                None
            }
            Self::Single(value) => Some((Bound::Included(value), Bound::Included(value))),
            Self::Inclusive(start, end) => Some((Bound::Included(start), Bound::Included(end))),
            Self::Exclusive(start, end) => Some((Bound::Excluded(start), Bound::Excluded(end))),
            Self::StartExclusive(start, end) => {
                Some((Bound::Excluded(start), Bound::Included(end)))
            }
            Self::EndExclusive(start, end) => Some((Bound::Included(start), Bound::Excluded(end))),
            Self::From(start) => Some((Bound::Included(start), Bound::Unbounded)),
            Self::FromExclusive(start) => Some((Bound::Excluded(start), Bound::Unbounded)),
            Self::To(end) => Some((Bound::Unbounded, Bound::Included(end))),
            Self::ToExclusive(end) => Some((Bound::Unbounded, Bound::Excluded(end))),
            Self::Full => Some((Bound::Unbounded, Bound::Unbounded)),
        }
    }

    #[must_use]
    pub fn start(&self) -> ContinuousBound<&Idx> {
        match self {
            Self::Empty => None,
            Self::Single(value) => Some(Bound::Included(value)),
            Self::Inclusive(start, _) => Some(Bound::Included(start)),
            Self::Exclusive(start, _) => Some(Bound::Excluded(start)),
            Self::StartExclusive(start, _) => Some(Bound::Excluded(start)),
            Self::EndExclusive(start, _) => Some(Bound::Included(start)),
            Self::From(start) => Some(Bound::Included(start)),
            Self::FromExclusive(start) => Some(Bound::Excluded(start)),
            Self::To(_) | Self::ToExclusive(_) | Self::Full => Some(Bound::Unbounded),
        }
        .into()
    }

    #[must_use]
    pub fn end(&self) -> ContinuousBound<&Idx> {
        match self {
            Self::Empty => None,
            Self::Single(value) => Some(Bound::Included(value)),
            Self::Inclusive(_, end) => Some(Bound::Included(end)),
            Self::Exclusive(_, end) => Some(Bound::Excluded(end)),
            Self::StartExclusive(_, end) => Some(Bound::Included(end)),
            Self::EndExclusive(_, end) => Some(Bound::Excluded(end)),
            Self::To(end) => Some(Bound::Included(end)),
            Self::ToExclusive(end) => Some(Bound::Excluded(end)),
            Self::From(_) | Self::FromExclusive(_) | Self::Full => Some(Bound::Unbounded),
        }
        .into()
    }

    /// Check if the range contains the provide value
    #[must_use]
    pub fn contains(&self, value: impl Borrow<Idx>) -> bool
    where
        Idx: PartialOrd + PartialEq,
    {
        match self {
            Self::Empty => false,
            Self::Single(single_value) => single_value == value.borrow(),
            Self::Inclusive(start, end) => {
                let value = value.borrow();
                value >= start && value <= end
            }
            Self::Exclusive(start, end) => {
                let value = value.borrow();
                value > start && value < end
            }
            Self::StartExclusive(start, end) => {
                let value = value.borrow();
                value > start && value <= end
            }
            Self::EndExclusive(start, end) => {
                let value = value.borrow();
                value >= start && value < end
            }
            Self::From(start) => value.borrow() >= start,
            Self::FromExclusive(start) => value.borrow() > start,
            Self::To(end) => value.borrow() <= end,
            Self::ToExclusive(end) => value.borrow() < end,
            Self::Full => true,
        }
    }

    #[must_use]
    pub fn union(&self, other: &ContinuousRange<Idx>) -> Option<ContinuousRange<Idx>>
    where
        Idx: PartialOrd + std::fmt::Debug,
    {
        match (self, other) {
            (ContinuousRange::Empty, r) | (r, ContinuousRange::Empty) => Some(r.clone()),
            (ContinuousRange::Full, _) | (_, ContinuousRange::Full) => Some(ContinuousRange::Full),
            _ => match self.compare(other) {
                Some(cmp) => match cmp {
                    RangesRelation::StrictlyBefore => None,
                    RangesRelation::StrictlyAfter => None,
                    RangesRelation::Meets => {
                        let start = self
                            .start()
                            .into_inner()
                            .expect("Self meets without bounds");
                        let end = other
                            .end()
                            .into_inner()
                            .expect("Other meets without bounds");
                        Some(ContinuousRange::from_bounds((start, end)))
                    }
                    RangesRelation::IsMet => {
                        let end = self.end().expect("Self meets without bounds");
                        let start = other.start().expect("Other meets without bounds");
                        Some(ContinuousRange::from_bounds((start, end)))
                    }
                    RangesRelation::Overlaps => {
                        let start = self.start().expect("Self meets without bounds");
                        let end = other.end().expect("Other meets without bounds");
                        Some(ContinuousRange::from_bounds((start, end)))
                    }
                    RangesRelation::IsOverlapped => {
                        let end = self.end().expect("Self meets without bounds");
                        let start = other.start().expect("Other meets without bounds");
                        Some(ContinuousRange::from_bounds((start, end)))
                    }
                    RangesRelation::Starts => Some(other.clone()),
                    RangesRelation::IsStarted => Some(self.clone()),
                    RangesRelation::StrictlyContains => Some(self.clone()),
                    RangesRelation::IsStrictlyContained => Some(other.clone()),
                    RangesRelation::Finishes => Some(other.clone()),
                    RangesRelation::IsFinished => Some(self.clone()),
                    RangesRelation::Equal => Some(self.clone()),
                },
                None => None,
            },
        }
    }

    #[must_use]
    pub fn intersection(&self, other: &ContinuousRange<Idx>) -> ContinuousRange<Idx>
    where
        Idx: PartialOrd + std::fmt::Debug,
    {
        match (self, other) {
            (ContinuousRange::Empty, _) | (_, ContinuousRange::Empty) => ContinuousRange::Empty,
            (ContinuousRange::Full, r) | (r, ContinuousRange::Full) => r.clone(),
            _ => match self.compare(other) {
                Some(cmp) => match cmp {
                    RangesRelation::StrictlyBefore => ContinuousRange::Empty,
                    RangesRelation::StrictlyAfter => ContinuousRange::Empty,
                    RangesRelation::Meets => {
                        let end = self.end().value().expect("Self meets without end bound");
                        ContinuousRange::single(end.clone())
                    }
                    RangesRelation::IsMet => {
                        let start = self
                            .start()
                            .value()
                            .expect("Self is met without start bound");
                        ContinuousRange::single(start.clone())
                    }
                    RangesRelation::Overlaps => {
                        let (_, end) = self.range_bounds().expect("Self overlaps without bounds");
                        let (start, _) = other
                            .range_bounds()
                            .expect("Other is overlapped without bounds");
                        ContinuousRange::from_bounds((start, end))
                    }
                    RangesRelation::IsOverlapped => {
                        let (start, _) = self
                            .range_bounds()
                            .expect("Self is overlapped without bounds");
                        let (_, end) = other.range_bounds().expect("Other overlaps without bounds");
                        ContinuousRange::from_bounds((start, end))
                    }
                    RangesRelation::Starts => self.clone(),
                    RangesRelation::IsStarted => other.clone(),

                    RangesRelation::StrictlyContains => other.clone(),
                    RangesRelation::IsStrictlyContained => self.clone(),
                    RangesRelation::Finishes => self.clone(),
                    RangesRelation::IsFinished => other.clone(),
                    RangesRelation::Equal => self.clone(),
                },
                None => ContinuousRange::Empty,
            },
        }
    }

    #[must_use]
    pub fn difference(&self, other: &ContinuousRange<Idx>) -> Option<ContinuousRange<Idx>>
    where
        Idx: PartialOrd + std::fmt::Debug,
    {
        match (self, other) {
            (ContinuousRange::Empty, r) => Some(r.clone()),
            (_, ContinuousRange::Empty) => Some(ContinuousRange::Empty),
            _ => match self.compare(other) {
                Some(cmp) => match cmp {
                    RangesRelation::StrictlyBefore => Some(self.clone()),
                    RangesRelation::StrictlyAfter => Some(self.clone()),
                    RangesRelation::Equal => Some(ContinuousRange::Empty),
                    RangesRelation::IsStrictlyContained => Some(ContinuousRange::Empty),
                    RangesRelation::StrictlyContains => None,

                    RangesRelation::Meets => {
                        let (start, end) = self.range_bounds().expect("Self meets without bounds");
                        let end = reverse_bound(end);
                        Some(ContinuousRange::from_bounds((start, end)))
                    }
                    RangesRelation::IsMet => {
                        let (start, end) = self.range_bounds().expect("Self is met without bounds");
                        let start = reverse_bound(start);
                        Some(ContinuousRange::from_bounds((start, end)))
                    }
                    RangesRelation::Overlaps => {
                        let start = self.start().expect("Self overlaps without bounds");
                        let end = other.start().expect("Other is overlapped without bounds");
                        let end = reverse_bound(end);
                        Some(ContinuousRange::from_bounds((start, end)))
                    }
                    RangesRelation::IsOverlapped => {
                        let end = self.end().expect("Self is overlapped without bounds");
                        let start = other.end().expect("Other overlaps without bounds");
                        let start = reverse_bound(start);
                        Some(ContinuousRange::from_bounds((start, end)))
                    }
                    RangesRelation::Starts => Some(ContinuousRange::Empty),
                    RangesRelation::IsStarted => {
                        let end = self.end().expect("Self is overlapped without bounds");
                        let start = other.end().expect("Other overlaps without bounds");
                        let start = reverse_bound(start);
                        Some(ContinuousRange::from_bounds((start, end)))
                    }
                    RangesRelation::Finishes => Some(ContinuousRange::Empty),
                    RangesRelation::IsFinished => {
                        let start = self.start().expect("Self overlaps without bounds");
                        let end = other.start().expect("Other is overlapped without bounds");
                        let end = reverse_bound(end);
                        Some(ContinuousRange::from_bounds((start, end)))
                    }
                },
                None => None,
            },
        }
    }

    #[must_use]
    pub fn intersects(&self, other: &ContinuousRange<Idx>) -> bool
    where
        Idx: std::fmt::Debug,
    {
        // Two empty ranges are 'equal' but we don't want to return true for them
        if self.is_empty() && other.is_empty() {
            false
        } else {
            match self.compare(other) {
                Some(relation) => relation.intersects(),
                None => false,
            }
        }
    }

    #[must_use]
    /// Compare the bounds of two ranges
    ///
    /// # Panics
    ///
    /// This function may panic if the [`PartialOrd`] contract isn't respected.
    pub fn compare(&self, other: &ContinuousRange<Idx>) -> Option<RangesRelation>
    where
        Idx: std::fmt::Debug,
    {
        // Inspired by "Maintaining Knowledge about Temporal Intervals" by James F. Allen
        // Communications of the ACM - November 1983 - Volume 26 - Number 11

        // Empty ranges don't have bounds so we need to special case them before anything else
        if self.is_empty() {
            return if other.is_empty() {
                Some(RangesRelation::Equal)
            } else {
                None
            };
        } else if other.is_empty() {
            return None;
        }

        let (self_start, self_end) = self
            .range_bounds()
            .expect("Non-empty self should have bounds");
        let (other_start, other_end) = other
            .range_bounds()
            .expect("Non-empty other should have bounds");

        let cmp_end_start =
            partial_cmp_bounds(&self_end, BoundSide::End, &other_start, BoundSide::Start)?;

        if cmp_end_start == Ordering::Less {
            return Some(RangesRelation::StrictlyBefore);
        }

        let cmp_start_end =
            partial_cmp_bounds(&self_start, BoundSide::Start, &other_end, BoundSide::End)?;

        if cmp_start_end == Ordering::Greater {
            return Some(RangesRelation::StrictlyAfter);
        }

        let self_cmp =
            partial_cmp_bounds(&self_start, BoundSide::Start, &self_end, BoundSide::End)?;

        let other_cmp =
            partial_cmp_bounds(&other_start, BoundSide::Start, &other_end, BoundSide::End)?;

        if cmp_end_start == Ordering::Equal
            && self_cmp != Ordering::Equal
            && other_cmp != Ordering::Equal
        {
            return Some(RangesRelation::Meets);
        }
        if cmp_start_end == Ordering::Equal
            && self_cmp != Ordering::Equal
            && other_cmp != Ordering::Equal
        {
            return Some(RangesRelation::IsMet);
        }

        let cmp_start_start = partial_cmp_bounds(
            &self_start,
            BoundSide::Start,
            &other_start,
            BoundSide::Start,
        )?;

        let cmp_end_end =
            partial_cmp_bounds(&self_end, BoundSide::End, &other_end, BoundSide::End)?;

        if cmp_start_start == Ordering::Less
            && cmp_end_start == Ordering::Greater
            && cmp_end_end == Ordering::Less
        {
            return Some(RangesRelation::Overlaps);
        }
        if cmp_start_start == Ordering::Greater
            && cmp_start_end == Ordering::Less
            && cmp_end_end == Ordering::Greater
        {
            return Some(RangesRelation::IsOverlapped);
        }
        if cmp_start_start == Ordering::Equal && cmp_end_end == Ordering::Less {
            return Some(RangesRelation::Starts);
        }
        if cmp_start_start == Ordering::Equal && cmp_end_end == Ordering::Greater {
            return Some(RangesRelation::IsStarted);
        }
        if cmp_start_start == Ordering::Greater && cmp_end_end == Ordering::Equal {
            return Some(RangesRelation::Finishes);
        }
        if cmp_start_start == Ordering::Less && cmp_end_end == Ordering::Equal {
            return Some(RangesRelation::IsFinished);
        }
        if cmp_start_start == Ordering::Less && cmp_end_end == Ordering::Greater {
            return Some(RangesRelation::StrictlyContains);
        }
        if cmp_start_start == Ordering::Greater && cmp_end_end == Ordering::Less {
            return Some(RangesRelation::IsStrictlyContained);
        }
        if cmp_start_start == Ordering::Equal && cmp_end_end == Ordering::Equal {
            return Some(RangesRelation::Equal);
        }

        // Should be unreachable if PartialOrd contract is correctly implemented
        panic!(
            r#"PartialOrd contract isn't correctly implemented.
No ordering can be found between {self:?} and {other:?}"#,
            self = &self,
            other = &other
        );
    }

    pub fn simplify_mut(&mut self)
    where
        Idx: PartialOrd,
    {
        match self {
            ContinuousRange::Inclusive(start, end) => {
                if start == end {
                    *self = ContinuousRange::Single(start.clone());
                } else if start > end {
                    *self = ContinuousRange::Empty;
                }
            }
            ContinuousRange::Exclusive(start, end)
            | ContinuousRange::StartExclusive(start, end)
            | ContinuousRange::EndExclusive(start, end) => {
                if start >= end {
                    *self = ContinuousRange::Empty;
                }
            }
            ContinuousRange::Empty
            | ContinuousRange::Single(_)
            | ContinuousRange::From(_)
            | ContinuousRange::FromExclusive(_)
            | ContinuousRange::To(_)
            | ContinuousRange::ToExclusive(_)
            | ContinuousRange::Full => {}
        }
    }

    #[must_use]
    pub fn simplify(&self) -> Self
    where
        Idx: PartialOrd,
    {
        let mut clone = (*self).clone();
        clone.simplify_mut();
        clone
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,

            // bounded ranges with inverted bounds are considered empty
            Self::Inclusive(start, end) => start > end,
            Self::Exclusive(start, end)
            | Self::StartExclusive(start, end)
            | Self::EndExclusive(start, end) => start >= end,

            Self::Single(_)

            // unbounded ranges can't be empty
            | Self::From(_)
            | Self::FromExclusive(_)
            | Self::To(_)
            | Self::ToExclusive(_)
            | Self::Full => false,
        }
    }

    #[must_use]
    pub fn is_full(&self) -> bool {
        match self {
            Self::Full => true,
            _ => false,
        }
    }
}

impl<Idx: PartialOrd + Clone> From<()> for ContinuousRange<Idx> {
    fn from(_: ()) -> Self {
        Self::empty()
    }
}

impl<Idx: PartialOrd + Clone> From<ops::RangeFull> for ContinuousRange<Idx> {
    fn from(_: ops::RangeFull) -> Self {
        Self::full()
    }
}

impl<Idx: PartialOrd + Clone> From<ops::Range<Idx>> for ContinuousRange<Idx> {
    fn from(r: ops::Range<Idx>) -> Self {
        Self::end_exclusive(r.start, r.end)
    }
}

impl<Idx: PartialOrd + Clone> From<ops::RangeInclusive<Idx>> for ContinuousRange<Idx> {
    fn from(r: ops::RangeInclusive<Idx>) -> Self {
        let (start, end) = r.into_inner();
        Self::inclusive(start, end)
    }
}

impl<Idx: PartialOrd + Clone> From<ops::RangeFrom<Idx>> for ContinuousRange<Idx> {
    fn from(r: ops::RangeFrom<Idx>) -> Self {
        Self::from(r.start)
    }
}

impl<Idx: PartialOrd + Clone> From<ops::RangeToInclusive<Idx>> for ContinuousRange<Idx> {
    fn from(r: ops::RangeToInclusive<Idx>) -> Self {
        Self::to(r.end)
    }
}

impl<Idx: PartialOrd + Clone> From<ops::RangeTo<Idx>> for ContinuousRange<Idx> {
    fn from(r: ops::RangeTo<Idx>) -> Self {
        Self::to_exclusive(r.end)
    }
}

impl<Idx: fmt::Debug> fmt::Debug for ContinuousRange<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContinuousRange::Empty => write!(fmt, "[]")?,
            ContinuousRange::Single(value) => value.fmt(fmt)?,
            ContinuousRange::Full => write!(fmt, "(..)")?,
            ContinuousRange::Inclusive(start, end) => {
                write!(fmt, "[")?;
                start.fmt(fmt)?;
                write!(fmt, "..")?;
                end.fmt(fmt)?;
                write!(fmt, "]")?;
            }
            ContinuousRange::Exclusive(start, end) => {
                write!(fmt, "(")?;
                start.fmt(fmt)?;
                write!(fmt, "..")?;
                end.fmt(fmt)?;
                write!(fmt, ")")?;
            }
            ContinuousRange::StartExclusive(start, end) => {
                write!(fmt, "(")?;
                start.fmt(fmt)?;
                write!(fmt, "..")?;
                end.fmt(fmt)?;
                write!(fmt, "]")?;
            }
            ContinuousRange::EndExclusive(start, end) => {
                write!(fmt, "[")?;
                start.fmt(fmt)?;
                write!(fmt, "..")?;
                end.fmt(fmt)?;
                write!(fmt, ")")?;
            }
            ContinuousRange::From(start) => {
                write!(fmt, "[")?;
                start.fmt(fmt)?;
                write!(fmt, "..")?;
                write!(fmt, ")")?;
            }
            ContinuousRange::FromExclusive(start) => {
                write!(fmt, "(")?;
                start.fmt(fmt)?;
                write!(fmt, "..")?;
                write!(fmt, ")")?;
            }
            ContinuousRange::To(end) => {
                write!(fmt, "(")?;
                write!(fmt, "..")?;
                end.fmt(fmt)?;
                write!(fmt, "]")?;
            }
            ContinuousRange::ToExclusive(end) => {
                write!(fmt, "(")?;
                write!(fmt, "..")?;
                end.fmt(fmt)?;
                write!(fmt, ")")?;
            }
        }
        Ok(())
    }
}

impl<Idx> Default for ContinuousRange<Idx> {
    fn default() -> Self {
        Self::Empty
    }
}
