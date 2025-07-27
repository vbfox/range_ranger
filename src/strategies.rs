use proptest::prelude::*;

use crate::ContinuousRange;

pub fn continuous_range_with_value_strategy<T>() -> BoxedStrategy<ContinuousRange<T>>
where T: Arbitrary + Clone + 'static {
    prop_oneof![
        any::<T>().prop_map(|v| ContinuousRange::Single(v)),
        any::<T>().prop_map(|v| ContinuousRange::From(v)),
        any::<T>().prop_map(|v| ContinuousRange::FromExclusive(v)),
        any::<T>().prop_map(|v| ContinuousRange::To(v)),
        any::<T>().prop_map(|v| ContinuousRange::ToExclusive(v)),
        (any::<T>(), any::<T>()).prop_map(|(s, e)| ContinuousRange::Inclusive(s, e)),
        (any::<T>(), any::<T>()).prop_map(|(s, e)| ContinuousRange::Exclusive(s, e)),
        (any::<T>(), any::<T>()).prop_map(|(s, e)| ContinuousRange::StartExclusive(s, e)),
        (any::<T>(), any::<T>()).prop_map(|(s, e)| ContinuousRange::EndExclusive(s, e)),
    ]
    .boxed()
}

pub fn continuous_range_non_empty_strategy<T>() -> BoxedStrategy<ContinuousRange<T>>
where T: Arbitrary + Clone + 'static {
    prop_oneof![
        1 => Just(ContinuousRange::Full),
        9 => continuous_range_with_value_strategy::<T>()
    ]
    .boxed()
}

pub fn continuous_range_strategy<T>() -> BoxedStrategy<ContinuousRange<T>>
where T: Arbitrary + Clone + 'static {
    prop_oneof![
        1 => Just(ContinuousRange::Empty),
        10 => continuous_range_non_empty_strategy::<T>()
    ]
    .boxed()
}
