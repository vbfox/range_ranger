use std::{cmp::Ordering, ops::Bound};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum BoundSide {
    Start,
    End,
}

pub fn partial_cmp_bounds<Idx: PartialOrd>(
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
