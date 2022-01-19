// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::language::*;

use alloc::string::String;
use alloc::string::ToString;
use icu_provider::DataPayload;
use icu_segmenter_lstm::lstm::Lstm;
use icu_segmenter_lstm::structs;

// TODO:
// json file is big, So I should use anoher binary format like npy.
// But provided npy uses tensorflow dtype.
const THAI_MODEL: &[u8; 373466] =
    include_bytes!("../tests/testdata/json/core/segmenter_lstm@1/th.json");
const BURMESE_MODEL: &[u8; 475209] =
    include_bytes!("../tests/testdata/json/core/segmenter_lstm@1/my.json");

lazy_static! {
    static ref THAI_LSTM: structs::LstmData<'static> =
        serde_json::from_slice(THAI_MODEL).expect("JSON syntax error");
    static ref BURMESE_LSTM: structs::LstmData<'static> =
        serde_json::from_slice(BURMESE_MODEL).expect("JSON syntax error");
}

// LSTM model depends on language, So we have to switch models per language.
pub fn get_best_lstm_model(codepoint: u32) -> Option<Lstm> {
    // TODO:
    // DataPayLoad isn't thread safe. We need anything static version.
    let lang = get_language(codepoint);
    match lang {
        Language::Thai => Some(Lstm::try_new(DataPayload::from_owned(THAI_LSTM.clone())).unwrap()),
        Language::Burmese => {
            Some(Lstm::try_new(DataPayload::from_owned(BURMESE_LSTM.clone())).unwrap())
        }
        _ => None,
    }
}

// A word break iterator using LSTM model. Input string have to be same language.

pub struct LstmSegmenterIterator {
    input: String,
    bies_str: String,
    pos: usize,
    pos_utf8: usize,
}

impl Iterator for LstmSegmenterIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let ch = self.bies_str.chars().nth(self.pos)?;
            self.pos_utf8 += self.input.chars().nth(self.pos).unwrap().len_utf8();
            self.pos += 1;
            if ch == 'e' && self.bies_str.len() > self.pos {
                return Some(self.pos_utf8);
            }
        }
    }
}

impl LstmSegmenterIterator {
    pub fn new(lstm: &Lstm, input: &str) -> Self {
        let lstm_output = lstm.word_segmenter(input);
        Self {
            input: input.to_string(),
            bies_str: lstm_output,
            pos: 0,
            pos_utf8: 0,
        }
    }
}

pub struct LstmSegmenterIteratorUtf16 {
    bies_str: String,
    pos: usize,
}

impl Iterator for LstmSegmenterIteratorUtf16 {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let ch = self.bies_str.chars().nth(self.pos)?;
            // This ch is always in bitmap.
            self.pos += 1;
            if ch == 'e' && self.bies_str.len() > self.pos {
                return Some(self.pos);
            }
        }
    }
}

impl LstmSegmenterIteratorUtf16 {
    pub fn new(lstm: &Lstm, input: &str) -> Self {
        let lstm_output = lstm.word_segmenter(input);
        Self {
            bies_str: lstm_output,
            pos: 0,
        }
    }
}
