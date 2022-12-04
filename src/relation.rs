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

    ///*
    /// ```text
    /// [ A ]
    ///     [ B ]
    /// ```
    ////
    Meets,

    ///*
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
}
