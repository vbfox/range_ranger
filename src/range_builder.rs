use crate::ContinuousRange;

pub struct RangeBuilder<Idx>(Vec<ContinuousRange<Idx>>);

impl<Idx> RangeBuilder<Idx> {
    pub fn new(ranges: Vec<ContinuousRange<Idx>>) -> Self {
        Self(ranges)
    }

    pub fn add(&mut self, range: ContinuousRange<Idx>) {}
}

impl<Idx> From<ContinuousRange<Idx>> for RangeBuilder<Idx> {
    fn from(r: ContinuousRange<Idx>) -> Self {
        Self(vec![r])
    }
}
