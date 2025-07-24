use std::fmt::Debug;

use crate::{ContinuousRange, RangesRelation};

/// Simplify a list of ranges, this operation produce mutate the input such that it satisfies the following:
///
/// - All continuous ranges are simplified
/// - Does not contain any empty ranges
/// - None of the range overlaps
/// - None of the ranges are joinable (The end of one is the start of the other, and they aren't both exclusive)
/// - They are ordered
pub fn simplify_ranges<Idx>(ranges: &mut Vec<ContinuousRange<Idx>>)
where
    Idx: PartialOrd + Clone + Debug,
{
    if ranges.is_empty() {
        return;
    }

    // 1. Simplify continuous ranges
    let mut index_to_remove = Vec::<usize>::new();

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
    for index in &index_to_remove {
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
}
