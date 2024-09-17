// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;
use potential_utf::PotentialUtf8;
use zerovec::{ZeroMap, ZeroVec};

#[derive(Debug, PartialEq, Clone, yoke::Yokeable)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_segmenter::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
#[non_exhaustive]
pub struct BudouXData<'data> {
    pub(crate) base_score: i32,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) uw1: ZeroMap<'data, PotentialUtf8, i32>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) uw2: ZeroMap<'data, PotentialUtf8, i32>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) uw3: ZeroMap<'data, PotentialUtf8, i32>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) uw4: ZeroMap<'data, PotentialUtf8, i32>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) uw5: ZeroMap<'data, PotentialUtf8, i32>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) uw6: ZeroMap<'data, PotentialUtf8, i32>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) bw1: ZeroMap<'data, PotentialUtf8, i32>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) bw2: ZeroMap<'data, PotentialUtf8, i32>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) bw3: ZeroMap<'data, PotentialUtf8, i32>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) tw1: ZeroMap<'data, PotentialUtf8, i32>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) tw2: ZeroMap<'data, PotentialUtf8, i32>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) tw3: ZeroMap<'data, PotentialUtf8, i32>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(crate) tw4: ZeroMap<'data, PotentialUtf8, i32>,
}

impl<'data> BudouXData<'data> {
    #[cfg(any(feature = "serde", feature = "datagen"))]
    /// Creates a BudouXData with the given data. Fails if the matrix dimensions are inconsistent.
    pub fn try_from_parts(
        base_score: i32,
        uw1: ZeroMap<'data, PotentialUtf8, i32>,
        uw2: ZeroMap<'data, PotentialUtf8, i32>,
        uw3: ZeroMap<'data, PotentialUtf8, i32>,
        uw4: ZeroMap<'data, PotentialUtf8, i32>,
        uw5: ZeroMap<'data, PotentialUtf8, i32>,
        uw6: ZeroMap<'data, PotentialUtf8, i32>,
        bw1: ZeroMap<'data, PotentialUtf8, i32>,
        bw2: ZeroMap<'data, PotentialUtf8, i32>,
        bw3: ZeroMap<'data, PotentialUtf8, i32>,
        tw1: ZeroMap<'data, PotentialUtf8, i32>,
        tw2: ZeroMap<'data, PotentialUtf8, i32>,
        tw3: ZeroMap<'data, PotentialUtf8, i32>,
        tw4: ZeroMap<'data, PotentialUtf8, i32>,
    ) -> Result<Self, DataError> {
        Ok(Self {
            base_score,
            uw1,
            uw2,
            uw3,
            uw4,
            uw5,
            uw6,
            bw1,
            bw2,
            bw3,
            tw1,
            tw2,
            tw3,
            tw4,
        })
    }
}

icu_provider::data_struct!(
    BudouXData<'_>,
    #[cfg(feature = "datagen")]
);
