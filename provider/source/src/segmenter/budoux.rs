// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by LSTM segmentation data.

use crate::{IterableDataProviderCached, SourceDataProvider};
use icu::locale::langid;
use icu::segmenter::provider::{BudouXData, SegmenterBudouxAutoV1};
use icu_provider::prelude::*;
use potential_utf::PotentialUtf8;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use zerovec::ZeroMap;

// BudouX JSON data structure.
#[allow(non_snake_case)]
#[derive(serde::Deserialize, Debug)]
struct RawBudouXData {
    UW1: HashMap<String, i32>,
    UW2: HashMap<String, i32>,
    UW3: HashMap<String, i32>,
    UW4: HashMap<String, i32>,
    UW5: HashMap<String, i32>,
    UW6: HashMap<String, i32>,
    BW1: HashMap<String, i32>,
    BW2: HashMap<String, i32>,
    BW3: HashMap<String, i32>,
    TW1: HashMap<String, i32>,
    TW2: HashMap<String, i32>,
    TW3: HashMap<String, i32>,
    TW4: HashMap<String, i32>,
}

impl RawBudouXData {
    pub(crate) fn try_convert(&self) -> Result<BudouXData<'static>, DataError> {
        let base_score = self.UW1.values().fold(0, |sum, x| sum + x)
            + self.UW2.values().fold(0, |sum, x| sum + x)
            + self.UW3.values().fold(0, |sum, x| sum + x)
            + self.UW4.values().fold(0, |sum, x| sum + x)
            + self.UW5.values().fold(0, |sum, x| sum + x)
            + self.UW6.values().fold(0, |sum, x| sum + x)
            + self.BW1.values().fold(0, |sum, x| sum + x)
            + self.BW2.values().fold(0, |sum, x| sum + x)
            + self.BW3.values().fold(0, |sum, x| sum + x)
            + self.TW1.values().fold(0, |sum, x| sum + x)
            + self.TW2.values().fold(0, |sum, x| sum + x)
            + self.TW3.values().fold(0, |sum, x| sum + x)
            + self.TW4.values().fold(0, |sum, x| sum + x);
        let data = BudouXData::try_from_parts(
            -base_score / 2,
            self.UW1
                .iter()
                .map(|(k, &v)| (PotentialUtf8::from_str(k), v))
                .collect(),
            self.UW2
                .iter()
                .map(|(k, &v)| (PotentialUtf8::from_str(k), v))
                .collect(),
            self.UW3
                .iter()
                .map(|(k, &v)| (PotentialUtf8::from_str(k), v))
                .collect(),
            self.UW4
                .iter()
                .map(|(k, &v)| (PotentialUtf8::from_str(k), v))
                .collect(),
            self.UW5
                .iter()
                .map(|(k, &v)| (PotentialUtf8::from_str(k), v))
                .collect(),
            self.UW6
                .iter()
                .map(|(k, &v)| (PotentialUtf8::from_str(k), v))
                .collect(),
            self.BW1
                .iter()
                .map(|(k, &v)| (PotentialUtf8::from_str(k), v))
                .collect(),
            self.BW2
                .iter()
                .map(|(k, &v)| (PotentialUtf8::from_str(k), v))
                .collect(),
            self.BW3
                .iter()
                .map(|(k, &v)| (PotentialUtf8::from_str(k), v))
                .collect(),
            self.TW1
                .iter()
                .map(|(k, &v)| (PotentialUtf8::from_str(k), v))
                .collect(),
            self.TW2
                .iter()
                .map(|(k, &v)| (PotentialUtf8::from_str(k), v))
                .collect(),
            self.TW3
                .iter()
                .map(|(k, &v)| (PotentialUtf8::from_str(k), v))
                .collect(),
            self.TW4
                .iter()
                .map(|(k, &v)| (PotentialUtf8::from_str(k), v))
                .collect(),
        )
        .map_err(|_| DataError::custom("Just checked the shapes"))?;
        Ok(data)
    }
}

impl DataProvider<SegmenterBudouxAutoV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<SegmenterBudouxAutoV1>, DataError> {
        self.check_req::<SegmenterBudouxAutoV1>(req)?;

        let data = self
            .segmenter_budoux()?
            .read_and_parse_json::<RawBudouXData>("budoux-0.6.2/budoux/models/ja.json")
            .map_err(|_| DataErrorKind::IdentifierNotFound.into_error())?;

        let data = data.try_convert()?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(data),
        })
    }
}

impl IterableDataProviderCached<SegmenterBudouxAutoV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        const SUPPORTED: [&DataMarkerAttributes; 1] =
            [DataMarkerAttributes::from_str_or_panic("budoux")];
        Ok(SUPPORTED
            .into_iter()
            .map(DataIdentifierCow::from_marker_attributes)
            .collect())
    }
}
