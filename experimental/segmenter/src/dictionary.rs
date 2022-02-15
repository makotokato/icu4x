// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::indices::Utf16Indices;
use crate::provider::*;
use core::str::CharIndices;
use icu_char16trie::char16trie::{Char16Trie, TrieResult};
use icu_provider::prelude::*;

/// A trait for dictionary based iterator
pub trait DictionaryType<'l, 's> {
    /// The iterator over characters.
    type IterAttr: Iterator<Item = (usize, Self::CharType)> + Clone;

    /// The character type.
    type CharType: Copy + Into<u32>;

    fn to_char(c: Self::CharType) -> char;
    fn char_len(c: Self::CharType) -> usize;
}

#[derive(Clone)]
pub struct DictionaryBreakIterator<'l, 's, Y: DictionaryType<'l, 's> + ?Sized> {
    trie: Char16Trie<'l>,
    iter: Y::IterAttr,
    len: usize,
    // TODO transform value for byte trie
}

impl<'l, 's, Y: DictionaryType<'l, 's> + ?Sized> Iterator for DictionaryBreakIterator<'l, 's, Y> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut trie_iter = self.trie.iter();
        let mut intermediate_length = 0;
        let mut not_match = false;
        let mut previous_match = None;

        loop {
            let next = self.iter.next();
            if next.is_none() {
                if intermediate_length > 0 {
                    return Some(intermediate_length);
                }
                // no match by scanning text
                if not_match {
                    return Some(self.len);
                }
                return None;
            }
            let next = next.unwrap();
            let ch = Y::to_char(next.1);
            let result = trie_iter.next(ch);
            match result {
                TrieResult::FinalValue(_) => {
                    return Some(next.0 + Y::char_len(next.1));
                }
                TrieResult::Intermediate(_) => {
                    intermediate_length = next.0 + Y::char_len(next.1);
                    previous_match = Some(self.iter.clone());
                }
                TrieResult::NoMatch => {
                    if intermediate_length > 0 {
                        if previous_match.is_some() {
                            self.iter = previous_match.unwrap();
                        }
                        return Some(intermediate_length);
                    }
                    // Not found
                    return Some(next.0 + Y::char_len(next.1));
                }
                TrieResult::NoValue => {
                    // Prefix string is matched
                    not_match = true;
                }
            }
        }
    }
}

impl<'l, 's> DictionaryType<'l, 's> for u32 {
    type IterAttr = Utf16Indices<'s>;
    type CharType = u32;

    fn to_char(c: u32) -> char {
        char::from_u32(c).unwrap_or(char::REPLACEMENT_CHARACTER)
    }

    fn char_len(c: u32) -> usize {
        if c >= 0x10000 {
            2
        } else {
            1
        }
    }
}

impl<'l, 's> DictionaryType<'l, 's> for char {
    type IterAttr = CharIndices<'s>;
    type CharType = char;

    fn to_char(c: char) -> char {
        c
    }

    fn char_len(c: char) -> usize {
        c.len_utf8()
    }
}

pub struct DictionarySegmenter {
    payload: DataPayload<UCharDictionaryBreakDataV1Marker>,
}

impl DictionarySegmenter {
    pub fn try_new(
        payload: DataPayload<UCharDictionaryBreakDataV1Marker>,
    ) -> Result<Self, DataError> {
        // TODO: no way to verify trie data
        Ok(Self { payload })
    }

    /// Create a dictionary based break iterator for an `str` (a UTF-8 string).
    pub fn segment_str<'l, 's>(&'l self, input: &'s str) -> DictionaryBreakIterator<'l, 's, char> {
        DictionaryBreakIterator {
            trie: Char16Trie::new(self.payload.get().trie_data.clone()),
            iter: input.char_indices(),
            len: input.len(),
        }
    }

    /// Create a dictionary based break iterator for a UTF-16 string.
    pub fn segment_utf16<'l, 's>(
        &'l self,
        input: &'s [u16],
    ) -> DictionaryBreakIterator<'l, 's, u32> {
        DictionaryBreakIterator {
            trie: Char16Trie::new(self.payload.get().trie_data.clone()),
            iter: Utf16Indices::new(input),
            len: input.len(),
        }
    }
}
