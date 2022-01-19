// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::String;
use alloc::string::ToString;
use core::str::Chars;

#[derive(PartialEq, PartialOrd, Ord, Eq)]
pub enum Language {
    Burmese,
    Khmer,
    Lao,
    Thai,
    Unknown,
}

pub fn get_language(codepoint: u32) -> Language {
    match codepoint {
        0xe01..=0xe7f => Language::Thai,
        0x1000..=0x109f => Language::Burmese,
        0x1780..=0x17ff => Language::Khmer,
        0xa9e0..=0xa9ff => Language::Burmese,
        0xaa60..=0xaa7f => Language::Burmese,

        _ => Language::Unknown,
    }
}

/// This struct is an iterator that returns the string per language from the
/// given string.
///
/// Actually supported LSTM model is Thai and Burmese only. If using other
/// code point, it causes panic.
pub struct LanguageIterator<'a> {
    input: Chars<'a>,
    last: Option<char>,
}

impl<'a> LanguageIterator<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut input = input.chars();
        let last = input.next();
        Self { input, last }
    }
}

impl<'a> Iterator for LanguageIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut s = "".to_string();

        let lang = get_language(self.last? as u32);
        s.push(self.last.unwrap());
        loop {
            let c = self.input.next();
            if c.is_none() {
                self.last = None;
                break;
            }
            self.last = c;
            let new_lang = get_language(c.unwrap() as u32);
            if lang != new_lang {
                break;
            }
            s.push(c.unwrap());
        }
        Some(s)
    }
}
