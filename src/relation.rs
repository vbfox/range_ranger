use std::cmp::Ordering;

/// How two different [`crate::ContinuousRange`] instances relate to each other.
///
/// This is based on [Allen's interval algebra](https://en.wikipedia.org/wiki/Allen%27s_interval_algebra) for temporal
/// intervals.
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum RangesRelation {
    /// The first range is strictly before the second one with no overlap
    ///
    /// ```text
    /// [ A ]
    ///       [ B ]
    /// ```
    StrictlyBefore,

    /// The first range is strictly after the second one with no overlap
    ///
    /// ```text
    ///       [ A ]
    /// [ B ]
    /// ```
    StrictlyAfter,

    /// The first range is before the second one and both ranges have a single common point that ends the first one and
    /// starts the second.
    /// ```text
    /// [ A ]
    ///     [ B ]
    /// ```
    ////
    Meets,

    /// The first range is after the seocnd one and both ranges have a single common point that ends the second one and
    /// starts the first.
    /// ```text
    ///     [ A ]
    /// [ B ]
    /// ```
    ////
    IsMet,

    ///*
    /// ```text
    /// [ A ]
    ///   [ B ]
    /// ```
    ////
    Overlaps,

    ///*
    /// ```text
    ///   [ A ]
    /// [ B ]
    /// ```
    ////
    IsOverlapped,

    ///*
    /// ```text
    /// [ A ]
    /// [   B   ]
    /// ```
    ////
    Starts,

    ///*
    /// ```text
    /// [   A   ]
    /// [ B ]
    /// ```
    ////
    IsStarted,

    ///*
    /// ```text
    /// [   A   ]
    ///   [ B ]
    /// ```
    ////
    StrictlyContains,

    ///*
    /// ```text
    ///   [ A ]
    /// [   B   ]
    /// ```
    ////
    IsStrictlyContained,

    ///*
    /// ```text
    ///     [ A ]
    /// [   B   ]
    /// ```
    ////
    Finishes,

    ///*
    /// ```text
    /// [   A   ]
    ///     [ B ]
    /// ```
    ////
    IsFinished,

    ///*
    /// ```text
    /// [ A ]
    /// [ B ]
    /// ```
    ////
    Equal,
}

impl RangesRelation {
    /// Returns true if there is any type of overlap between the two ranges
    ///
    /// This is equivalend to all the relations except:
    /// - [`RangesRelation::StrictlyBefore`]
    /// - [`RangesRelation::StrictlyAfter`]
    #[must_use]
    pub fn intersects(&self) -> bool {
        match self {
            RangesRelation::StrictlyBefore | RangesRelation::StrictlyAfter => false,

            RangesRelation::Overlaps
            | RangesRelation::IsOverlapped
            | RangesRelation::Meets
            | RangesRelation::IsMet
            | RangesRelation::Starts
            | RangesRelation::IsStarted
            | RangesRelation::StrictlyContains
            | RangesRelation::IsStrictlyContained
            | RangesRelation::Finishes
            | RangesRelation::IsFinished
            | RangesRelation::Equal => true,
        }
    }

    /// Returns true if the ranges are completely disjoint
    ///
    /// This is equivalend to the relations:
    /// - [`RangesRelation::StrictlyBefore`]
    /// - [`RangesRelation::StrictlyAfter`]
    #[must_use]
    pub fn disjoint(&self) -> bool {
        !self.intersects()
    }

    /// Returns true if the first range contains the second one.
    ///
    /// The equivalent relations are:
    /// - [`RangesRelation::Equal`]
    /// - [`RangesRelation::StrictlyContains`]
    /// - [`RangesRelation::Started`] / [`RangesRelation::Finished`]
    #[must_use]
    pub fn contains(&self) -> bool {
        match self {
            RangesRelation::Equal
            | RangesRelation::StrictlyContains
            | RangesRelation::IsFinished
            | RangesRelation::IsStarted => true,

            RangesRelation::StrictlyBefore
            | RangesRelation::StrictlyAfter
            | RangesRelation::Overlaps
            | RangesRelation::IsOverlapped
            | RangesRelation::Meets
            | RangesRelation::IsMet
            | RangesRelation::Starts
            | RangesRelation::IsStrictlyContained
            | RangesRelation::Finishes => false,
        }
    }

    /// Get the relative ordering of the start bound of the ranges
    #[must_use]
    pub fn start_ordering(&self) -> Ordering {
        match self {
            RangesRelation::StrictlyBefore => Ordering::Less,
            RangesRelation::StrictlyAfter => Ordering::Greater,
            RangesRelation::Meets => Ordering::Less,
            RangesRelation::IsMet => Ordering::Greater,
            RangesRelation::Overlaps => Ordering::Less,
            RangesRelation::IsOverlapped => Ordering::Greater,
            RangesRelation::Starts => Ordering::Equal,
            RangesRelation::IsStarted => Ordering::Equal,
            RangesRelation::StrictlyContains => Ordering::Less,
            RangesRelation::IsStrictlyContained => Ordering::Greater,
            RangesRelation::Finishes => Ordering::Greater,
            RangesRelation::IsFinished => Ordering::Less,
            RangesRelation::Equal => Ordering::Equal,
        }
    }

    /// Get the relative ordering of the end bound of the ranges
    #[must_use]
    pub fn end_ordering(&self) -> Ordering {
        match self {
            RangesRelation::StrictlyBefore => Ordering::Less,
            RangesRelation::StrictlyAfter => Ordering::Greater,
            RangesRelation::Meets => Ordering::Less,
            RangesRelation::IsMet => Ordering::Greater,
            RangesRelation::Overlaps => Ordering::Less,
            RangesRelation::IsOverlapped => Ordering::Greater,
            RangesRelation::Starts => Ordering::Less,
            RangesRelation::IsStarted => Ordering::Greater,
            RangesRelation::StrictlyContains => Ordering::Greater,
            RangesRelation::IsStrictlyContained => Ordering::Less,
            RangesRelation::Finishes => Ordering::Equal,
            RangesRelation::IsFinished => Ordering::Equal,
            RangesRelation::Equal => Ordering::Equal,
        }
    }
}
