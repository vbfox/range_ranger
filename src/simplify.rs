use std::fmt::Debug;

use crate::{ContinuousRange, RangesRelation};

/// Simplify a list of ranges, this operation produce mutate the input such that it satisfies the following:
///
/// 1. All continuous ranges are simplified
/// 2. Does not contain any empty ranges
/// 3. None of the range overlaps
/// 4. None of the ranges meets (The end of one is the start of the other, and they aren't both exclusive)
/// 5. They are ordered
pub fn simplify_ranges<Idx>(ranges: &mut Vec<ContinuousRange<Idx>>)
where
    Idx: PartialOrd + Clone + Debug,
{
    if ranges.is_empty() {
        return;
    }

    // 1. Simplify continuous ranges
    let mut index_to_remove = Vec::<usize>::new();

    println!("Ranges: {ranges:?}");
    for i in 0..ranges.len() {
        // Take the range to simplify it (Can't do it in place as it would require something like
        // std::mem::replace_with but it was considered unsound)
        let mut range = std::mem::take(&mut ranges[i]);
        range = range.simplify();

        match range {
            ContinuousRange::Empty => {
                // After simplification we don't need to call is_empty as the only possible empty range is
                // ContinuousRange::Empty.
                index_to_remove.push(i);
                // The array still contains the default value that `std::mem::take` placed there but we'll remove it
                // and anyway the default is the empty range.
            }
            ContinuousRange::Full => {
                // The result is always a full range, return that
                ranges[0] = ContinuousRange::Full;
                ranges.truncate(1);
                return;
            }
            _ => {
                // Put back the simplified value
                ranges[i] = range;
            }
        }
    }

    // 2. Remove empty ranges
    println!("To remove: {index_to_remove:?}");
    for index in index_to_remove.iter().rev() {
        ranges.swap_remove(*index);
    }

    // 3. Sort the range by their start values
    ranges.sort_by(|a, b| {
        a.compare(b)
            .expect("No empty range should be present")
            .start_ordering()
    });

    // 4. Merge the ranges, as they are already sorted we can consider a single range, and merge the next ones as long
    // as we don't see a 'hole'.
    let mut write_index = 0;
    let mut read_index = 1;
    let len = ranges.len();
    while read_index < len {
        let write = &ranges[write_index];
        let read = &ranges[read_index];

        match write
            .compare(read)
            .expect("No empty range should be present")
        {
            RangesRelation::StrictlyBefore => {
                // No overlaps, let's move the write head and put the read value there
                write_index += 1;
                ranges.swap(write_index, read_index);
                read_index += 1;
            }
            RangesRelation::StrictlyAfter => panic!("Unexpected range order after sort"),
            cmp @ (RangesRelation::Meets
            | RangesRelation::IsMet
            | RangesRelation::Overlaps
            | RangesRelation::IsOverlapped) => {
                ranges[write_index] = write
                    .union_knowing_cmp(read, cmp)
                    .expect("Unexpected failible union");
                read_index += 1;
            }
            RangesRelation::Starts
            | RangesRelation::IsStrictlyContained
            | RangesRelation::Finishes => {
                ranges.swap(write_index, read_index);
                read_index += 1;
            }
            RangesRelation::IsStarted
            | RangesRelation::StrictlyContains
            | RangesRelation::IsFinished
            | RangesRelation::Equal => {
                read_index += 1;
            }
        }
    }

    ranges.truncate(write_index + 1);
}

#[cfg(test)]
mod tests {
    use std::ops::Bound;

    use assertables::assert_all;
    use proptest::prelude::*;

    use crate::{bounds::expect_bound, strategies::continuous_range_strategy};

    use super::*;

    fn simplified_ranges<Idx>(ranges: Vec<ContinuousRange<Idx>>) -> Vec<ContinuousRange<Idx>>
    where
        Idx: PartialOrd + Clone + Debug,
    {
        let mut ranges = ranges;
        simplify_ranges(&mut ranges);
        ranges
    }

    #[test]
    pub fn empty_no_op() {
        let ranges = simplified_ranges::<i32>(vec![]);
        assert_eq!(ranges, vec![]);
    }

    #[test]
    pub fn remove_empty_range() {
        let ranges = simplified_ranges::<i32>(vec![ContinuousRange::empty()]);
        assert_eq!(ranges, vec![]);
    }

    #[test]
    pub fn simplify_continuous() {
        let ranges = simplified_ranges::<i32>(vec![ContinuousRange::Inclusive(42, 42)]);
        assert_eq!(ranges, vec![ContinuousRange::Single(42)]);
    }

    #[test]
    pub fn full_overlaps_everything() {
        let ranges = simplified_ranges::<i32>(vec![
            ContinuousRange::Single(42),
            ContinuousRange::Full,
            ContinuousRange::StartExclusive(1, 8),
        ]);
        assert_eq!(ranges, vec![ContinuousRange::Full]);
    }

    #[test]
    pub fn order_single() {
        let ranges =
            simplified_ranges::<i32>(vec![ContinuousRange::single(2), ContinuousRange::single(1)]);
        assert_eq!(
            ranges,
            vec![ContinuousRange::single(1), ContinuousRange::single(2)]
        );
    }

    #[test]
    pub fn complex() {
        let ranges = simplified_ranges::<i32>(vec![
            ContinuousRange::Single(42),
            ContinuousRange::Single(200),
            ContinuousRange::To(50),
            ContinuousRange::StartExclusive(1, 100),
        ]);
        assert_eq!(
            ranges,
            vec![ContinuousRange::To(100), ContinuousRange::Single(200),]
        );
    }

    #[test]
    pub fn proptest_repro_1() {
        let ranges = simplified_ranges::<i32>(vec![
            ContinuousRange::Single(0),
            ContinuousRange::Single(0),
            ContinuousRange::Empty,
            ContinuousRange::Empty,
        ]);
        assert_eq!(ranges, vec![ContinuousRange::Single(0)]);
    }

    #[test]
    pub fn proptest_repro_2() {
        let ranges = simplified_ranges::<i32>(vec![
            ContinuousRange::EndExclusive(0, 203),
            ContinuousRange::Single(203),
        ]);
        assert_eq!(ranges, vec![ContinuousRange::Inclusive(0, 203)]);
    }

    /// 2 ranges meet if they are joinable, so if they don't overlap but the inclusive end of one is the exclusive
    /// one of the other. Or vice-versa.
    fn meets<T>(a: &ContinuousRange<T>, b: &ContinuousRange<T>) -> bool
    where
        T: Ord,
    {
        let a_bounds = a.range_bounds();
        let b_bounds = b.range_bounds();

        match (a_bounds, b_bounds) {
            (Some(a_bounds), Some(b_bounds)) => {
                match (a_bounds.0, a_bounds.1, b_bounds.0, b_bounds.1) {
                    (_, Bound::Included(a_end), Bound::Excluded(b_start), _)
                    | (_, Bound::Excluded(a_end), Bound::Included(b_start), _)
                        if a_end == b_start =>
                    {
                        true
                    }
                    (Bound::Included(a_start), _, _, Bound::Excluded(b_end))
                    | (Bound::Excluded(a_start), _, _, Bound::Included(b_end))
                        if a_start == b_end =>
                    {
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn simplify_ranges_proptest_impl(
        ranges: Vec<ContinuousRange<u8>>,
    ) -> Result<(), TestCaseError> {
        let mut simplified_ranges = ranges.clone();
        simplify_ranges(&mut simplified_ranges);
        for (i, range) in simplified_ranges.iter().enumerate() {
            // 1. All continuous ranges are simplified
            let simplified = range.clone().simplify();
            prop_assert_eq!(&simplified, range);

            // 2. Does not contain any empty ranges
            prop_assert!(!range.is_empty());

            // 3. None of the range overlaps
            let others = simplified_ranges
                .iter()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .map(|(_, v)| v.clone())
                .collect::<Vec<_>>();
            assert_all!(others.iter(), |other| !range.intersects(other));

            // 4. None of the ranges meets
            assert_all!(others.iter(), |other| !meets(range, other));

            // 5. They are ordered
            if i > 0 {
                let before = simplified_ranges[i - 1];
                let self_start = expect_bound(range.start_bound(), "No self start bound");
                let before_end = expect_bound(before.end_bound(), "No other end bound");
                prop_assert!(
                    before_end <= self_start,
                    "{before:?}.end <= {range:?}.start"
                );

                // The check via compare ensure coherency and tests both the bound comparison and the "meets" condition
                prop_assert_eq!(range.compare(&before), Some(RangesRelation::StrictlyAfter));
            }
            if i < simplified_ranges.len() - 1 {
                let after = simplified_ranges[i + 1];
                let self_end = expect_bound(range.end_bound(), "No self end bound");
                let after_start = expect_bound(after.start_bound(), "No other start bound");
                prop_assert!(self_end <= after_start, "{range:?}.end <= {after:?}.end");

                // The check via compare ensure coherency and tests both the bound comparison and the "meets" condition
                prop_assert_eq!(range.compare(&after), Some(RangesRelation::StrictlyBefore));
            }
        }
        Ok(())
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 1000, .. ProptestConfig::default()
        })]

        #[test]
        fn simplify_ranges_proptest(ranges in prop::collection::vec(continuous_range_strategy(), 0..20)) {
            simplify_ranges_proptest_impl(ranges)?;
        }
    }
}
