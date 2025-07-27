use std::{cmp::Ordering, ops::Bound};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum BoundSide {
    Start,
    End,
}

/// A `BoundOrdering` is the result of a comparison between two [`Bound`].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i8)]
pub enum BoundOrdering {
    /// An ordering where a compared bound is less than another but they "meet" (the value is the same)
    Meets = -2,
    /// An ordering where a compared bound is less than another.
    Less = -1,
    /// An ordering where a compared bound is equal to another.
    Equal = 0,
    /// An ordering where a compared bound is greater than another.
    Greater = 1,
    /// An ordering where a compared bound is greater than another but they "meet" (the value is the same)
    IsMet = 2,
}

impl From<Ordering> for BoundOrdering {
    fn from(value: Ordering) -> Self {
        match value {
            Ordering::Less => BoundOrdering::Less,
            Ordering::Equal => BoundOrdering::Equal,
            Ordering::Greater => BoundOrdering::Greater,
        }
    }
}

/// Compare the bounds of 2 ranges
pub fn partial_cmp_bounds<Idx: PartialOrd>(
    this: &Bound<&Idx>,
    this_side: BoundSide,
    other: &Bound<&Idx>,
    other_side: BoundSide,
) -> Option<BoundOrdering> {
    match *this {
        Bound::Included(this_value) => match *other {
            Bound::Included(other_value) => this_value.partial_cmp(other_value).map(Ordering::into),
            Bound::Excluded(other_value) => match this_value.partial_cmp(other_value) {
                Some(Ordering::Equal) => match (this_side, other_side) {
                    (BoundSide::Start, BoundSide::Start) => Some(BoundOrdering::Less),
                    (BoundSide::End, BoundSide::End) => Some(BoundOrdering::Greater),
                    (BoundSide::Start, BoundSide::End) => Some(BoundOrdering::IsMet),
                    (BoundSide::End, BoundSide::Start) => Some(BoundOrdering::Meets),
                },
                other => other.map(Ordering::into),
            },
            Bound::Unbounded => match other_side {
                BoundSide::Start => Some(BoundOrdering::Greater), // -Inf
                BoundSide::End => Some(BoundOrdering::Less),      // +Inf
            },
        },
        Bound::Excluded(this_value) => match *other {
            Bound::Included(other_value) => match this_value.partial_cmp(other_value) {
                Some(Ordering::Equal) => match (this_side, other_side) {
                    (BoundSide::Start, BoundSide::Start) => Some(BoundOrdering::Greater),
                    (BoundSide::End, BoundSide::End) => Some(BoundOrdering::Less),
                    (BoundSide::Start, BoundSide::End) => Some(BoundOrdering::IsMet),
                    (BoundSide::End, BoundSide::Start) => Some(BoundOrdering::Meets),
                },
                other => other.map(Ordering::into),
            },
            Bound::Excluded(other_value) => match this_value.partial_cmp(other_value) {
                Some(Ordering::Equal) => match (this_side, other_side) {
                    (BoundSide::Start, BoundSide::Start) => Some(BoundOrdering::Equal),
                    (BoundSide::End, BoundSide::End) => Some(BoundOrdering::Equal),
                    (BoundSide::Start, BoundSide::End) => Some(BoundOrdering::Greater),
                    (BoundSide::End, BoundSide::Start) => Some(BoundOrdering::Less),
                },
                other => other.map(Ordering::into),
            },
            Bound::Unbounded => match other_side {
                BoundSide::Start => Some(BoundOrdering::Greater), // -Inf
                BoundSide::End => Some(BoundOrdering::Less),      // +Inf
            },
        },
        Bound::Unbounded => match other {
            Bound::Included(_) | Bound::Excluded(_) => match this_side {
                BoundSide::Start => Some(BoundOrdering::Less),  // -Inf
                BoundSide::End => Some(BoundOrdering::Greater), // +Inf
            },
            Bound::Unbounded => match (this_side, other_side) {
                (BoundSide::Start, BoundSide::Start) => Some(BoundOrdering::Equal),
                (BoundSide::End, BoundSide::End) => Some(BoundOrdering::Equal),
                (BoundSide::Start, BoundSide::End) => Some(BoundOrdering::Less),
                (BoundSide::End, BoundSide::Start) => Some(BoundOrdering::Greater),
            },
        },
    }
}

/// Get the Internal value of a bound or panics if [Unbounded][Bound::Unbounded].
pub fn expect_bound<'a, Idx>(bound: Option<Bound<&'a Idx>>, msg: &'static str) -> &'a Idx {
    match bound.expect(msg) {
        Bound::Included(x) => x,
        Bound::Excluded(x) => x,
        Bound::Unbounded => panic!("{}", msg),
    }
}

/// Reverse a bound between [`Bound::Included`] and [`Bound::Excluded`].
///
/// [`Bound::Unbounded`] is kept as-is.
pub fn reverse_bound<Idx>(bound: Bound<&Idx>) -> Bound<&Idx> {
    match bound {
        Bound::Included(x) => Bound::Excluded(x),
        Bound::Excluded(x) => Bound::Included(x),
        Bound::Unbounded => Bound::Unbounded,
    }
}
