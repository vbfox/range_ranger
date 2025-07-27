mod test_fmt_debug {
    use crate::ContinuousRange;

    #[test]
    pub fn empty() {
        let r = ContinuousRange::<i32>::Empty;
        assert_eq!(format!("{r:?}"), "[]");
    }

    #[test]
    pub fn single() {
        let r = ContinuousRange::Single(5);
        assert_eq!(format!("{r:?}"), "5");
    }

    #[test]
    pub fn inclusive() {
        let r = ContinuousRange::Inclusive(1, 5);
        assert_eq!(format!("{r:?}"), "[1..5]");
    }

    #[test]
    pub fn inclusive_equal() {
        let r = ContinuousRange::Inclusive(1, 1);
        assert_eq!(format!("{r:?}"), "[1..1]");
    }

    #[test]
    pub fn inclusive_inverted() {
        let r = ContinuousRange::Inclusive(5, 1);
        assert_eq!(format!("{r:?}"), "[5..1]");
    }

    #[test]
    pub fn exclusive() {
        let r = ContinuousRange::Exclusive(1, 5);
        assert_eq!(format!("{r:?}"), "(1..5)");
    }

    #[test]
    pub fn exclusive_equal() {
        let r = ContinuousRange::Exclusive(1, 1);
        assert_eq!(format!("{r:?}"), "(1..1)");
    }

    #[test]
    pub fn exclusive_inverted() {
        let r = ContinuousRange::Exclusive(5, 1);
        assert_eq!(format!("{r:?}"), "(5..1)");
    }

    #[test]
    pub fn start_exclusive() {
        let r = ContinuousRange::StartExclusive(1, 5);
        assert_eq!(format!("{r:?}"), "(1..5]");
    }

    #[test]
    pub fn start_exclusive_equal() {
        let r = ContinuousRange::StartExclusive(1, 1);
        assert_eq!(format!("{r:?}"), "(1..1]");
    }

    #[test]
    pub fn start_exclusive_inverted() {
        let r = ContinuousRange::StartExclusive(5, 1);
        assert_eq!(format!("{r:?}"), "(5..1]");
    }

    #[test]
    pub fn end_exclusive() {
        let r = ContinuousRange::EndExclusive(1, 5);
        assert_eq!(format!("{r:?}"), "[1..5)");
    }

    #[test]
    pub fn end_exclusive_equal() {
        let r = ContinuousRange::EndExclusive(1, 1);
        assert_eq!(format!("{r:?}"), "[1..1)");
    }

    #[test]
    pub fn end_exclusive_inverted() {
        let r = ContinuousRange::EndExclusive(5, 1);
        assert_eq!(format!("{r:?}"), "[5..1)");
    }

    #[test]
    pub fn full() {
        let r = ContinuousRange::<i32>::Full;
        assert_eq!(format!("{r:?}"), "(..)");
    }

    #[test]
    pub fn from() {
        let r = ContinuousRange::From(1);
        assert_eq!(format!("{r:?}"), "[1..)");
    }

    #[test]
    pub fn from_exclusive() {
        let r = ContinuousRange::FromExclusive(1);
        assert_eq!(format!("{r:?}"), "(1..)");
    }

    #[test]
    pub fn to() {
        let r = ContinuousRange::To(5);
        assert_eq!(format!("{r:?}"), "(..5]");
    }

    #[test]
    pub fn to_exclusive() {
        let r = ContinuousRange::ToExclusive(5);
        assert_eq!(format!("{r:?}"), "(..5)");
    }
}

#[allow(clippy::reversed_empty_ranges)]
mod test_from_stdlib {
    use assert_matches::assert_matches;

    use crate::ContinuousRange;

    #[test]
    pub fn default() {
        let r: ContinuousRange<i32> = ContinuousRange::default();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn empty() {
        let r: ContinuousRange<i32> = ().into();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn inclusive() {
        let r: ContinuousRange<_> = (1..=5).into();
        assert_matches!(r, ContinuousRange::Inclusive(1, 5));
    }

    #[test]
    pub fn inclusive_equal() {
        let r: ContinuousRange<_> = (1..=1).into();
        assert_matches!(r, ContinuousRange::Single(1));
    }

    #[test]
    pub fn inclusive_inverted() {
        let r: ContinuousRange<_> = (5..=1).into();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn end_exclusive() {
        let r: ContinuousRange<_> = (1..5).into();
        assert_eq!(r, ContinuousRange::EndExclusive(1, 5));
    }

    #[test]
    pub fn end_exclusive_equal() {
        let r: ContinuousRange<_> = (1..1).into();
        assert_eq!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn end_exclusive_inverted() {
        let r: ContinuousRange<_> = (5..1).into();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn from() {
        let r: ContinuousRange<_> = (1..).into();
        assert_matches!(r, ContinuousRange::From(1));
    }

    #[test]
    pub fn to() {
        let r: ContinuousRange<_> = (..=5).into();
        assert_matches!(r, ContinuousRange::To(5));
    }

    #[test]
    pub fn to_exclusive() {
        let r: ContinuousRange<_> = (..5).into();
        assert_matches!(r, ContinuousRange::ToExclusive(5));
    }

    #[test]
    pub fn full() {
        let r: ContinuousRange<i32> = (..).into();
        assert_matches!(r, ContinuousRange::Full);
    }
}

mod test_creation_functions {
    use assert_matches::assert_matches;

    use crate::ContinuousRange;

    #[test]
    pub fn empty() {
        let r = ContinuousRange::<i32>::empty();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn single() {
        let r = ContinuousRange::single(5);
        assert_matches!(r, ContinuousRange::Single(5));
    }

    #[test]
    pub fn inclusive() {
        let r = ContinuousRange::inclusive(1, 5);
        assert_matches!(r, ContinuousRange::Inclusive(1, 5));
    }

    #[test]
    pub fn inclusive_equal() {
        let r = ContinuousRange::inclusive(1, 1);
        assert_matches!(r, ContinuousRange::Single(1));
    }

    #[test]
    pub fn inclusive_inverted() {
        let r = ContinuousRange::inclusive(5, 1);
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn exclusive() {
        let r = ContinuousRange::exclusive(1, 5);
        assert_matches!(r, ContinuousRange::Exclusive(1, 5));
    }

    #[test]
    pub fn exclusive_equal() {
        let r = ContinuousRange::exclusive(1, 1);
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn exclusive_inverted() {
        let r = ContinuousRange::exclusive(5, 1);
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn start_exclusive() {
        let r = ContinuousRange::start_exclusive(1, 5);
        assert_matches!(r, ContinuousRange::StartExclusive(1, 5));
    }

    #[test]
    pub fn start_exclusive_equal() {
        let r = ContinuousRange::start_exclusive(1, 1);
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn start_exclusive_inverted() {
        let r = ContinuousRange::start_exclusive(5, 1);
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn end_exclusive() {
        let r = ContinuousRange::end_exclusive(1, 5);
        assert_matches!(r, ContinuousRange::EndExclusive(1, 5));
    }

    #[test]
    pub fn end_exclusive_equal() {
        let r = ContinuousRange::end_exclusive(1, 1);
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn end_exclusive_inverted() {
        let r = ContinuousRange::end_exclusive(5, 1);
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn from() {
        let r = ContinuousRange::from(1);
        assert_matches!(r, ContinuousRange::From(1));
    }

    #[test]
    pub fn from_exclusive() {
        let r = ContinuousRange::from_exclusive(5);
        assert_matches!(r, ContinuousRange::FromExclusive(5));
    }

    #[test]
    pub fn to() {
        let r = ContinuousRange::to(5);
        assert_matches!(r, ContinuousRange::To(5));
    }

    #[test]
    pub fn to_exclusive() {
        let r = ContinuousRange::to_exclusive(5);
        assert_matches!(r, ContinuousRange::ToExclusive(5));
    }

    #[test]
    pub fn full() {
        let r = ContinuousRange::<i32>::full();
        assert_matches!(r, ContinuousRange::Full);
    }
}

mod test_is_full {
    use crate::ContinuousRange;

    #[test]
    pub fn empty() {
        let r = ContinuousRange::<i32>::Empty;
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn single() {
        let r = ContinuousRange::Single(5);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn inclusive() {
        let r = ContinuousRange::Inclusive(1, 5);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn inclusive_equal() {
        let r = ContinuousRange::Inclusive(1, 1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn inclusive_inverted() {
        let r = ContinuousRange::Inclusive(5, 1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn exclusive() {
        let r = ContinuousRange::Exclusive(1, 5);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn exclusive_equal() {
        let r = ContinuousRange::Exclusive(1, 1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn exclusive_inverted() {
        let r = ContinuousRange::Exclusive(5, 1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn start_exclusive() {
        let r = ContinuousRange::StartExclusive(1, 5);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn start_exclusive_equal() {
        let r = ContinuousRange::StartExclusive(1, 1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn start_exclusive_inverted() {
        let r = ContinuousRange::StartExclusive(5, 1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn end_exclusive() {
        let r = ContinuousRange::EndExclusive(1, 5);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn end_exclusive_equal() {
        let r = ContinuousRange::EndExclusive(1, 1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn end_exclusive_inverted() {
        let r = ContinuousRange::EndExclusive(5, 1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn full() {
        let r = ContinuousRange::<i32>::Full;
        assert_eq!(r.is_full(), true);
    }

    #[test]
    pub fn from() {
        let r = ContinuousRange::From(1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn from_exclusive() {
        let r = ContinuousRange::FromExclusive(1);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn to() {
        let r = ContinuousRange::To(5);
        assert_eq!(r.is_full(), false);
    }

    #[test]
    pub fn to_exclusive() {
        let r = ContinuousRange::ToExclusive(5);
        assert_eq!(r.is_full(), false);
    }
}

mod test_is_empty {
    use crate::ContinuousRange;

    #[test]
    pub fn empty() {
        let r = ContinuousRange::<i32>::Empty;
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn single() {
        let r = ContinuousRange::Single(5);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn inclusive() {
        let r = ContinuousRange::Inclusive(1, 5);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn inclusive_equal() {
        let r = ContinuousRange::Inclusive(1, 1);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn inclusive_inverted() {
        let r = ContinuousRange::Inclusive(5, 1);
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn exclusive() {
        let r = ContinuousRange::Exclusive(1, 5);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn exclusive_equal() {
        let r = ContinuousRange::Exclusive(1, 1);
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn exclusive_inverted() {
        let r = ContinuousRange::Exclusive(5, 1);
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn start_exclusive() {
        let r = ContinuousRange::StartExclusive(1, 5);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn start_exclusive_equal() {
        let r = ContinuousRange::StartExclusive(1, 1);
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn start_exclusive_inverted() {
        let r = ContinuousRange::StartExclusive(5, 1);
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn end_exclusive() {
        let r = ContinuousRange::EndExclusive(1, 5);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn end_exclusive_equal() {
        let r = ContinuousRange::EndExclusive(1, 1);
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn end_exclusive_inverted() {
        let r = ContinuousRange::EndExclusive(5, 1);
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    pub fn full() {
        let r = ContinuousRange::<i32>::Full;
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn from() {
        let r = ContinuousRange::From(1);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn from_exclusive() {
        let r = ContinuousRange::FromExclusive(1);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn to() {
        let r = ContinuousRange::To(5);
        assert_eq!(r.is_empty(), false);
    }

    #[test]
    pub fn to_exclusive() {
        let r = ContinuousRange::ToExclusive(5);
        assert_eq!(r.is_empty(), false);
    }
}

mod test_simplify {
    use assert_matches::assert_matches;

    use crate::ContinuousRange;

    #[test]
    pub fn empty() {
        let r = ContinuousRange::<i32>::Empty.simplify();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn single() {
        let r = ContinuousRange::Single(5).simplify();
        assert_matches!(r, ContinuousRange::Single(5));
    }

    #[test]
    pub fn inclusive() {
        let r = ContinuousRange::Inclusive(1, 5).simplify();
        assert_matches!(r, ContinuousRange::Inclusive(1, 5));
    }

    #[test]
    pub fn inclusive_equal() {
        let r = ContinuousRange::Inclusive(1, 1).simplify();
        assert_matches!(r, ContinuousRange::Single(1));
    }

    #[test]
    pub fn inclusive_inverted() {
        let r = ContinuousRange::Inclusive(5, 1).simplify();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn exclusive() {
        let r = ContinuousRange::Exclusive(1, 5).simplify();
        assert_matches!(r, ContinuousRange::Exclusive(1, 5));
    }

    #[test]
    pub fn exclusive_equal() {
        let r = ContinuousRange::Exclusive(1, 1).simplify();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn exclusive_inverted() {
        let r = ContinuousRange::Exclusive(5, 1).simplify();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn start_exclusive() {
        let r = ContinuousRange::StartExclusive(1, 5).simplify();
        assert_matches!(r, ContinuousRange::StartExclusive(1, 5));
    }

    #[test]
    pub fn start_exclusive_equal() {
        let r = ContinuousRange::StartExclusive(1, 1).simplify();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn start_exclusive_inverted() {
        let r = ContinuousRange::StartExclusive(5, 1).simplify();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn end_exclusive() {
        let r = ContinuousRange::EndExclusive(1, 5).simplify();
        assert_matches!(r, ContinuousRange::EndExclusive(1, 5));
    }

    #[test]
    pub fn end_exclusive_equal() {
        let r = ContinuousRange::EndExclusive(1, 1).simplify();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn end_exclusive_inverted() {
        let r = ContinuousRange::EndExclusive(5, 1).simplify();
        assert_matches!(r, ContinuousRange::Empty);
    }

    #[test]
    pub fn from() {
        let r = ContinuousRange::From(1).simplify();
        assert_matches!(r, ContinuousRange::From(1));
    }

    #[test]
    pub fn from_exclusive() {
        let r = ContinuousRange::FromExclusive(5).simplify();
        assert_matches!(r, ContinuousRange::FromExclusive(5));
    }

    #[test]
    pub fn to() {
        let r = ContinuousRange::To(5).simplify();
        assert_matches!(r, ContinuousRange::To(5));
    }

    #[test]
    pub fn to_exclusive() {
        let r = ContinuousRange::ToExclusive(5).simplify();
        assert_matches!(r, ContinuousRange::ToExclusive(5));
    }

    #[test]
    pub fn full() {
        let r = ContinuousRange::<i32>::Full.simplify();
        assert_matches!(r, ContinuousRange::Full);
    }
}

mod test_contains {
    use crate::ContinuousRange;

    // i32::MAX didn't exist in our MSRV version
    pub const MAX_I32: i32 = 2_147_483_647i32;

    #[test]
    pub fn empty() {
        let r: ContinuousRange<i32> = ContinuousRange::empty();
        assert_eq!(r.contains(-500), false);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(42), false);
        assert_eq!(r.contains(MAX_I32), false);
    }

    #[test]
    pub fn single() {
        let r = ContinuousRange::Single(42);
        assert_eq!(r.contains(-500), false);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(42), true);
        assert_eq!(r.contains(MAX_I32), false);
    }

    #[test]
    pub fn inclusive() {
        let r = ContinuousRange::Inclusive(1, 5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), true);
        assert_eq!(r.contains(3), true);
        assert_eq!(r.contains(5), true);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn inclusive_equal() {
        let r = ContinuousRange::Inclusive(5, 5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), true);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn inclusive_inverted() {
        let r = ContinuousRange::Inclusive(5, 1);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn exclusive() {
        let r = ContinuousRange::Exclusive(1, 5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), true);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn exclusive_equal() {
        let r = ContinuousRange::Exclusive(5, 5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn exclusive_inverted() {
        let r = ContinuousRange::Exclusive(5, 1);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn start_exclusive() {
        let r = ContinuousRange::StartExclusive(1, 5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), true);
        assert_eq!(r.contains(5), true);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn start_exclusive_equal() {
        let r = ContinuousRange::StartExclusive(5, 5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn start_exclusive_inverted() {
        let r = ContinuousRange::StartExclusive(5, 1);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn end_exclusive() {
        let r = ContinuousRange::EndExclusive(1, 5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), true);
        assert_eq!(r.contains(3), true);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn end_exclusive_equal() {
        let r = ContinuousRange::EndExclusive(5, 5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn end_exclusive_inverted() {
        let r = ContinuousRange::EndExclusive(5, 1);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(1), false);
        assert_eq!(r.contains(3), false);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(42), false);
    }

    #[test]
    pub fn from() {
        let r = ContinuousRange::From(5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(5), true);
        assert_eq!(r.contains(10), true);
    }

    #[test]
    pub fn from_exclusive() {
        let r = ContinuousRange::FromExclusive(5);
        assert_eq!(r.contains(0), false);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(6), true);
        assert_eq!(r.contains(10), true);
    }

    #[test]
    pub fn to() {
        let r = ContinuousRange::To(5);
        assert_eq!(r.contains(0), true);
        assert_eq!(r.contains(5), true);
        assert_eq!(r.contains(10), false);
    }

    #[test]
    pub fn to_exclusive() {
        let r = ContinuousRange::ToExclusive(5);
        assert_eq!(r.contains(0), true);
        assert_eq!(r.contains(4), true);
        assert_eq!(r.contains(5), false);
        assert_eq!(r.contains(10), false);
    }

    #[test]
    pub fn full() {
        let r = ContinuousRange::<i32>::Full;
        assert_eq!(r.contains(-500), true);
        assert_eq!(r.contains(0), true);
        assert_eq!(r.contains(42), true);
        assert_eq!(r.contains(MAX_I32), true);
    }
}

mod test_compare {
    use crate::{ContinuousRange, RangesRelation};

    macro_rules! compare {
        ($a:expr, $b:expr, $relation:expr) => {
            let a = $a;
            let b = $b;
            assert_eq!(a.compare(&b), $relation, "Comparing {:?} and {:?}", a, b);
        };
    }

    #[test]
    pub fn empty() {
        compare!(
            ContinuousRange::<i32>::Empty,
            ContinuousRange::<i32>::Empty,
            Some(RangesRelation::Equal)
        );
        compare!(
            ContinuousRange::<i32>::Empty,
            ContinuousRange::Inclusive(5, 1),
            Some(RangesRelation::Equal)
        );
        compare!(
            ContinuousRange::<i32>::Empty,
            ContinuousRange::Inclusive(1, 5),
            None
        );
        compare!(
            ContinuousRange::Inclusive(1, 5),
            ContinuousRange::<i32>::Empty,
            None
        );
        compare!(
            ContinuousRange::Inclusive(1, 5),
            ContinuousRange::Inclusive(5, 1),
            None
        );
    }

    #[test]
    pub fn full() {
        compare!(
            ContinuousRange::<i32>::Full,
            ContinuousRange::<i32>::Full,
            Some(RangesRelation::Equal)
        );
        compare!(
            ContinuousRange::<i32>::Full,
            ContinuousRange::<i32>::Empty,
            None
        );
        compare!(
            ContinuousRange::<i32>::Full,
            ContinuousRange::Inclusive(1, 5),
            Some(RangesRelation::StrictlyContains)
        );
        compare!(
            ContinuousRange::Inclusive(1, 5),
            ContinuousRange::<i32>::Full,
            Some(RangesRelation::IsStrictlyContained)
        );
    }

    #[test]
    pub fn strictly_before_and_after() {
        macro_rules! strictly_before {
            ($a:expr, $b:expr) => {
                compare!($a, $b, Some(RangesRelation::StrictlyBefore));
                compare!($b, $a, Some(RangesRelation::StrictlyAfter));
            };
        }

        strictly_before!(
            ContinuousRange::Single(1),
            ContinuousRange::Inclusive(6, 10)
        );
        strictly_before!(ContinuousRange::Single(1), ContinuousRange::Single(5));
        strictly_before!(ContinuousRange::Inclusive(1, 4), ContinuousRange::Single(5));
        strictly_before!(
            ContinuousRange::EndExclusive(0, 1),
            ContinuousRange::Exclusive(1, 10)
        );
        strictly_before!(
            ContinuousRange::EndExclusive(1, 5),
            ContinuousRange::StartExclusive(5, 15)
        );

        strictly_before!(
            ContinuousRange::Inclusive(1, 5),
            ContinuousRange::Inclusive(6, 10)
        );
        strictly_before!(
            ContinuousRange::Exclusive(1, 5),
            ContinuousRange::Exclusive(6, 10)
        );
        strictly_before!(
            ContinuousRange::Exclusive(1, 5),
            ContinuousRange::Exclusive(5, 10)
        );

        strictly_before!(ContinuousRange::Inclusive(1, 5), ContinuousRange::From(6));
        strictly_before!(ContinuousRange::To(5), ContinuousRange::From(6));
        strictly_before!(ContinuousRange::To(5), ContinuousRange::Inclusive(6, 10));
        strictly_before!(
            ContinuousRange::EndExclusive(1, 6),
            ContinuousRange::FromExclusive(6)
        );
        strictly_before!(
            ContinuousRange::ToExclusive(5),
            ContinuousRange::StartExclusive(5, 10)
        );
    }

    #[test]
    pub fn strictly_contains_and_is_strictly_contained() {
        macro_rules! strictly_contains {
            ($a:expr, $b:expr) => {
                compare!($a, $b, Some(RangesRelation::StrictlyContains));
                compare!($b, $a, Some(RangesRelation::IsStrictlyContained));
            };
        }

        strictly_contains!(
            ContinuousRange::Inclusive(0, 10),
            ContinuousRange::Single(1)
        );
        strictly_contains!(
            ContinuousRange::Exclusive(0, 10),
            ContinuousRange::Single(5)
        );

        strictly_contains!(
            ContinuousRange::Inclusive(1, 10),
            ContinuousRange::Inclusive(2, 9)
        );
        strictly_contains!(
            ContinuousRange::Inclusive(1, 10),
            ContinuousRange::Exclusive(2, 9)
        );
        strictly_contains!(
            ContinuousRange::Inclusive(1, 10),
            ContinuousRange::Exclusive(1, 10)
        );
        strictly_contains!(
            ContinuousRange::Inclusive(1, 10),
            ContinuousRange::StartExclusive(2, 9)
        );
        strictly_contains!(
            ContinuousRange::Inclusive(1, 10),
            ContinuousRange::StartExclusive(1, 9)
        );
        strictly_contains!(
            ContinuousRange::Inclusive(1, 10),
            ContinuousRange::EndExclusive(2, 9)
        );
        strictly_contains!(
            ContinuousRange::Inclusive(1, 10),
            ContinuousRange::EndExclusive(2, 10)
        );

        strictly_contains!(ContinuousRange::From(0), ContinuousRange::Inclusive(2, 10));
        strictly_contains!(ContinuousRange::From(0), ContinuousRange::Exclusive(2, 10));
        strictly_contains!(ContinuousRange::From(0), ContinuousRange::Exclusive(0, 10));
        strictly_contains!(
            ContinuousRange::From(0),
            ContinuousRange::StartExclusive(2, 10)
        );
        strictly_contains!(
            ContinuousRange::From(0),
            ContinuousRange::StartExclusive(0, 10)
        );
        strictly_contains!(
            ContinuousRange::From(0),
            ContinuousRange::EndExclusive(2, 10)
        );

        strictly_contains!(ContinuousRange::To(20), ContinuousRange::Inclusive(0, 19));
        strictly_contains!(ContinuousRange::To(20), ContinuousRange::Exclusive(0, 19));
        strictly_contains!(ContinuousRange::To(20), ContinuousRange::Exclusive(0, 20));
        strictly_contains!(
            ContinuousRange::To(20),
            ContinuousRange::StartExclusive(0, 19)
        );
        strictly_contains!(
            ContinuousRange::To(20),
            ContinuousRange::EndExclusive(0, 19)
        );
        strictly_contains!(
            ContinuousRange::To(20),
            ContinuousRange::EndExclusive(0, 20)
        );
    }

    #[test]
    pub fn overlaps_and_is_overlapped() {
        macro_rules! overlaps {
            ($a:expr, $b:expr) => {
                compare!($a, $b, Some(RangesRelation::Overlaps));
                compare!($b, $a, Some(RangesRelation::IsOverlapped));
            };
        }

        overlaps!(
            ContinuousRange::Inclusive(0, 10),
            ContinuousRange::Inclusive(5, 20)
        );
        overlaps!(
            ContinuousRange::Inclusive(0, 10),
            ContinuousRange::Inclusive(10, 20)
        );
        overlaps!(
            ContinuousRange::Exclusive(0, 10),
            ContinuousRange::Inclusive(5, 20)
        );
        overlaps!(
            ContinuousRange::Inclusive(0, 10),
            ContinuousRange::Exclusive(5, 20)
        );
        overlaps!(
            ContinuousRange::Exclusive(0, 10),
            ContinuousRange::Exclusive(5, 20)
        );

        overlaps!(
            ContinuousRange::Inclusive(0, 10),
            ContinuousRange::Exclusive(9, 11)
        );

        overlaps!(ContinuousRange::To(10), ContinuousRange::Inclusive(5, 20));
        overlaps!(
            ContinuousRange::ToExclusive(10),
            ContinuousRange::Inclusive(5, 20)
        );

        overlaps!(ContinuousRange::Inclusive(0, 10), ContinuousRange::From(5));
        overlaps!(
            ContinuousRange::Inclusive(0, 10),
            ContinuousRange::FromExclusive(5)
        );

        overlaps!(ContinuousRange::To(10), ContinuousRange::From(5));
    }

    #[test]
    pub fn meet_and_is_met() {
        macro_rules! meet {
            ($a:expr, $b:expr) => {
                compare!($a, $b, Some(RangesRelation::Meets));
                compare!($b, $a, Some(RangesRelation::IsMet));
            };
        }

        meet!(
            ContinuousRange::Inclusive(0, 10),
            ContinuousRange::Exclusive(10, 20)
        );
        meet!(
            ContinuousRange::Inclusive(0, 10),
            ContinuousRange::StartExclusive(10, 20)
        );
        meet!(
            ContinuousRange::EndExclusive(0, 10),
            ContinuousRange::Inclusive(10, 20)
        );
        meet!(
            ContinuousRange::EndExclusive(0, 10),
            ContinuousRange::EndExclusive(10, 20)
        );

        meet!(
            ContinuousRange::To(10),
            ContinuousRange::StartExclusive(10, 20)
        );
        meet!(ContinuousRange::To(10), ContinuousRange::Exclusive(10, 20));
        meet!(
            ContinuousRange::ToExclusive(10),
            ContinuousRange::Inclusive(10, 20)
        );
        meet!(
            ContinuousRange::ToExclusive(10),
            ContinuousRange::EndExclusive(10, 20)
        );

        meet!(
            ContinuousRange::EndExclusive(0, 10),
            ContinuousRange::From(10)
        );
        meet!(ContinuousRange::Exclusive(0, 10), ContinuousRange::From(10));
        meet!(
            ContinuousRange::Inclusive(0, 10),
            ContinuousRange::FromExclusive(10)
        );
        meet!(
            ContinuousRange::StartExclusive(0, 10),
            ContinuousRange::FromExclusive(10)
        );
    }

    #[test]
    pub fn starts_and_is_started() {
        macro_rules! starts {
            ($a:expr, $b:expr) => {
                compare!($a, $b, Some(RangesRelation::Starts));
                compare!($b, $a, Some(RangesRelation::IsStarted));
            };
        }

        starts!(
            ContinuousRange::Inclusive(0, 10),
            ContinuousRange::Inclusive(0, 20)
        );
        starts!(
            ContinuousRange::EndExclusive(0, 10),
            ContinuousRange::Inclusive(0, 20)
        );

        starts!(
            ContinuousRange::Exclusive(0, 10),
            ContinuousRange::Exclusive(0, 20)
        );

        starts!(
            ContinuousRange::Exclusive(0, 10),
            ContinuousRange::StartExclusive(0, 20)
        );
    }

    #[test]
    pub fn finishes_and_is_finished() {
        macro_rules! finishes {
            ($a:expr, $b:expr) => {
                compare!($a, $b, Some(RangesRelation::Finishes));
                compare!($b, $a, Some(RangesRelation::IsFinished));
            };
        }

        finishes!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::Inclusive(0, 20)
        );
        finishes!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::StartExclusive(0, 20)
        );

        finishes!(
            ContinuousRange::Exclusive(10, 20),
            ContinuousRange::Exclusive(0, 20)
        );

        finishes!(
            ContinuousRange::Exclusive(10, 20),
            ContinuousRange::EndExclusive(0, 20)
        );
    }
}

mod test_union {
    use crate::ContinuousRange;

    macro_rules! union {
        ($a:expr, $b:expr, $c:expr) => {
            assert_eq!($a.union(&$b), $c);
        };
    }

    #[test]
    pub fn empty() {
        union!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::Empty,
            Some(ContinuousRange::Inclusive(10, 20))
        );
        union!(
            ContinuousRange::Empty,
            ContinuousRange::Inclusive(10, 20),
            Some(ContinuousRange::Inclusive(10, 20))
        );
    }

    #[test]
    pub fn full() {
        union!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::Full,
            Some(ContinuousRange::Full)
        );
        union!(
            ContinuousRange::Full,
            ContinuousRange::Inclusive(10, 20),
            Some(ContinuousRange::Full)
        );

        union!(
            ContinuousRange::<i32>::Full,
            ContinuousRange::Empty,
            Some(ContinuousRange::Full)
        );
        union!(
            ContinuousRange::Empty,
            ContinuousRange::<i32>::Full,
            Some(ContinuousRange::Full)
        );
    }

    #[test]
    pub fn strictly_before() {
        union!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::Inclusive(42, 100),
            None
        );
    }

    #[test]
    pub fn strictly_after() {
        union!(
            ContinuousRange::Inclusive(42, 100),
            ContinuousRange::Inclusive(10, 20),
            None
        );
    }

    #[test]
    pub fn overlaps_and_is_overlapped() {
        union!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::Inclusive(15, 100),
            Some(ContinuousRange::Inclusive(10, 100))
        );
        union!(
            ContinuousRange::Inclusive(15, 100),
            ContinuousRange::Inclusive(10, 20),
            Some(ContinuousRange::Inclusive(10, 100))
        );

        union!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::Exclusive(15, 100),
            Some(ContinuousRange::EndExclusive(10, 100))
        );
        union!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::EndExclusive(15, 100),
            Some(ContinuousRange::EndExclusive(10, 100))
        );
        union!(
            ContinuousRange::StartExclusive(10, 20),
            ContinuousRange::EndExclusive(15, 100),
            Some(ContinuousRange::Exclusive(10, 100))
        );
    }

    #[test]
    pub fn meets_and_is_met() {
        union!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::Inclusive(20, 100),
            Some(ContinuousRange::Inclusive(10, 100))
        );
        union!(
            ContinuousRange::Inclusive(20, 100),
            ContinuousRange::Inclusive(10, 20),
            Some(ContinuousRange::Inclusive(10, 100))
        );

        union!(
            ContinuousRange::StartExclusive(10, 20),
            ContinuousRange::Inclusive(20, 100),
            Some(ContinuousRange::StartExclusive(10, 100))
        );
        union!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::EndExclusive(20, 100),
            Some(ContinuousRange::EndExclusive(10, 100))
        );
        union!(
            ContinuousRange::StartExclusive(10, 20),
            ContinuousRange::EndExclusive(20, 100),
            Some(ContinuousRange::Exclusive(10, 100))
        );
    }

    #[test]
    pub fn starts_and_is_started() {
        union!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::Inclusive(10, 100),
            Some(ContinuousRange::Inclusive(10, 100))
        );
        union!(
            ContinuousRange::Inclusive(10, 100),
            ContinuousRange::Inclusive(10, 20),
            Some(ContinuousRange::Inclusive(10, 100))
        );

        union!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::From(10),
            Some(ContinuousRange::From(10))
        );
    }

    #[test]
    pub fn finishes_and_is_finished() {
        union!(
            ContinuousRange::Inclusive(99, 100),
            ContinuousRange::Inclusive(10, 100),
            Some(ContinuousRange::Inclusive(10, 100))
        );
        union!(
            ContinuousRange::Inclusive(10, 100),
            ContinuousRange::Inclusive(99, 100),
            Some(ContinuousRange::Inclusive(10, 100))
        );

        union!(
            ContinuousRange::Inclusive(5, 10),
            ContinuousRange::To(10),
            Some(ContinuousRange::To(10))
        );
    }

    #[test]
    pub fn contains_and_is_contained() {
        union!(
            ContinuousRange::Inclusive(0, 100),
            ContinuousRange::Inclusive(5, 10),
            Some(ContinuousRange::Inclusive(0, 100))
        );
        union!(
            ContinuousRange::Inclusive(5, 10),
            ContinuousRange::Inclusive(0, 100),
            Some(ContinuousRange::Inclusive(0, 100))
        );

        union!(
            ContinuousRange::Inclusive(5, 10),
            ContinuousRange::From(0),
            Some(ContinuousRange::From(0))
        );
        union!(
            ContinuousRange::Inclusive(5, 10),
            ContinuousRange::To(100),
            Some(ContinuousRange::To(100))
        );
    }

    #[test]
    pub fn equal() {
        union!(
            ContinuousRange::Inclusive(0, 100),
            ContinuousRange::Inclusive(0, 100),
            Some(ContinuousRange::Inclusive(0, 100))
        );
        union!(
            ContinuousRange::From(5),
            ContinuousRange::From(5),
            Some(ContinuousRange::From(5))
        );
        union!(
            ContinuousRange::To(5),
            ContinuousRange::To(5),
            Some(ContinuousRange::To(5))
        );
    }
}

mod test_start_end {
    use std::ops::Bound;

    use crate::ContinuousRange;

    #[test]
    pub fn empty() {
        let r = ContinuousRange::<i32>::Empty;
        assert_eq!(r.start(), None);
        assert_eq!(r.end(), None);
    }

    #[test]
    pub fn single() {
        let r = ContinuousRange::Single(42);
        assert_eq!(r.start(), Some(Bound::Included(&42)));
        assert_eq!(r.end(), Some(Bound::Included(&42)));
    }

    #[test]
    pub fn inclusive() {
        let r = ContinuousRange::Inclusive(0, 42);
        assert_eq!(r.start(), Some(Bound::Included(&0)));
        assert_eq!(r.end(), Some(Bound::Included(&42)));
    }

    #[test]
    pub fn exclusive() {
        let r = ContinuousRange::Exclusive(0, 42);
        assert_eq!(r.start(), Some(Bound::Excluded(&0)));
        assert_eq!(r.end(), Some(Bound::Excluded(&42)));
    }

    #[test]
    pub fn start_exclusive() {
        let r = ContinuousRange::StartExclusive(0, 42);
        assert_eq!(r.start(), Some(Bound::Excluded(&0)));
        assert_eq!(r.end(), Some(Bound::Included(&42)));
    }

    #[test]
    pub fn end_exclusive() {
        let r = ContinuousRange::EndExclusive(0, 42);
        assert_eq!(r.start(), Some(Bound::Included(&0)));
        assert_eq!(r.end(), Some(Bound::Excluded(&42)));
    }

    #[test]
    pub fn from() {
        let r = ContinuousRange::From(42);
        assert_eq!(r.start(), Some(Bound::Included(&42)));
        assert_eq!(r.end(), Some(Bound::Unbounded));
    }

    #[test]
    pub fn from_exclusive() {
        let r = ContinuousRange::FromExclusive(42);
        assert_eq!(r.start(), Some(Bound::Excluded(&42)));
        assert_eq!(r.end(), Some(Bound::Unbounded));
    }

    #[test]
    pub fn to() {
        let r = ContinuousRange::To(42);
        assert_eq!(r.start(), Some(Bound::Unbounded));
        assert_eq!(r.end(), Some(Bound::Included(&42)));
    }

    #[test]
    pub fn to_exclusive() {
        let r = ContinuousRange::ToExclusive(42);
        assert_eq!(r.start(), Some(Bound::Unbounded));
        assert_eq!(r.end(), Some(Bound::Excluded(&42)));
    }
}

mod test_intersection {
    use crate::ContinuousRange;

    macro_rules! intersection {
        ($a:expr, $b:expr, $c:expr) => {
            assert_eq!($a.intersection(&$b), $c);
        };
    }

    #[test]
    pub fn empty() {
        intersection!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::Empty,
            ContinuousRange::Empty
        );
        intersection!(
            ContinuousRange::Empty,
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::Empty
        );
    }

    #[test]
    pub fn full() {
        intersection!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::Full,
            ContinuousRange::Inclusive(10, 20)
        );
        intersection!(
            ContinuousRange::Full,
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::Inclusive(10, 20)
        );

        intersection!(
            ContinuousRange::<i32>::Full,
            ContinuousRange::Empty,
            ContinuousRange::Empty
        );
        intersection!(
            ContinuousRange::Empty,
            ContinuousRange::<i32>::Full,
            ContinuousRange::Empty
        );
    }

    #[test]
    pub fn strictly_before() {
        intersection!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::Inclusive(42, 100),
            ContinuousRange::Empty
        );
    }

    #[test]
    pub fn strictly_after() {
        intersection!(
            ContinuousRange::Inclusive(42, 100),
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::Empty
        );
    }

    #[test]
    pub fn overlaps_and_is_overlapped() {
        intersection!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::Inclusive(15, 100),
            ContinuousRange::Inclusive(15, 20)
        );
        intersection!(
            ContinuousRange::Inclusive(15, 100),
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::Inclusive(15, 20)
        );

        intersection!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::Exclusive(15, 100),
            ContinuousRange::StartExclusive(15, 20)
        );
        intersection!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::EndExclusive(15, 100),
            ContinuousRange::Inclusive(15, 20)
        );
        intersection!(
            ContinuousRange::EndExclusive(10, 20),
            ContinuousRange::StartExclusive(15, 100),
            ContinuousRange::Exclusive(15, 20)
        );
    }

    #[test]
    pub fn meets_and_is_met() {
        intersection!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::Inclusive(20, 100),
            ContinuousRange::Single(20)
        );
        intersection!(
            ContinuousRange::Inclusive(20, 100),
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::Single(20)
        );

        intersection!(
            ContinuousRange::StartExclusive(10, 20),
            ContinuousRange::Inclusive(20, 100),
            ContinuousRange::Single(20)
        );
        intersection!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::EndExclusive(20, 100),
            ContinuousRange::Single(20)
        );
        intersection!(
            ContinuousRange::StartExclusive(10, 20),
            ContinuousRange::EndExclusive(20, 100),
            ContinuousRange::Single(20)
        );
    }

    #[test]
    pub fn starts_and_is_started() {
        intersection!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::Inclusive(10, 100),
            ContinuousRange::Inclusive(10, 20)
        );
        intersection!(
            ContinuousRange::Inclusive(10, 100),
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::Inclusive(10, 20)
        );

        intersection!(
            ContinuousRange::Inclusive(10, 20),
            ContinuousRange::From(10),
            ContinuousRange::Inclusive(10, 20)
        );
    }

    #[test]
    pub fn finishes_and_is_finished() {
        intersection!(
            ContinuousRange::Inclusive(99, 100),
            ContinuousRange::Inclusive(10, 100),
            ContinuousRange::Inclusive(99, 100)
        );
        intersection!(
            ContinuousRange::Inclusive(10, 100),
            ContinuousRange::Inclusive(99, 100),
            ContinuousRange::Inclusive(99, 100)
        );

        intersection!(
            ContinuousRange::Inclusive(5, 10),
            ContinuousRange::To(10),
            ContinuousRange::Inclusive(5, 10)
        );
    }

    #[test]
    pub fn contains_and_is_contained() {
        intersection!(
            ContinuousRange::Inclusive(0, 100),
            ContinuousRange::Inclusive(5, 10),
            ContinuousRange::Inclusive(5, 10)
        );
        intersection!(
            ContinuousRange::Inclusive(5, 10),
            ContinuousRange::Inclusive(0, 100),
            ContinuousRange::Inclusive(5, 10)
        );

        intersection!(
            ContinuousRange::Inclusive(5, 10),
            ContinuousRange::From(0),
            ContinuousRange::Inclusive(5, 10)
        );
        intersection!(
            ContinuousRange::Inclusive(5, 10),
            ContinuousRange::To(100),
            ContinuousRange::Inclusive(5, 10)
        );
    }

    #[test]
    pub fn equal() {
        intersection!(
            ContinuousRange::Inclusive(0, 100),
            ContinuousRange::Inclusive(0, 100),
            ContinuousRange::Inclusive(0, 100)
        );
        intersection!(
            ContinuousRange::From(5),
            ContinuousRange::From(5),
            ContinuousRange::From(5)
        );
        intersection!(
            ContinuousRange::To(5),
            ContinuousRange::To(5),
            ContinuousRange::To(5)
        );
    }
}
