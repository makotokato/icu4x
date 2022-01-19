// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::language::*;

#[cfg(feature = "lstm")]
use crate::lstm::*;
use alloc::string::String;
use alloc::vec::Vec;
use core::char::decode_utf16;

pub fn segmenter_break_utf8(input: &str) -> Option<Vec<usize>> {
    let mut result: Vec<usize> = Vec::new();
    let mut offset = 0;
    for str_per_lang in LanguageIterator::new(input) {
        if offset != 0 {
            result.push(offset);
        }

        #[cfg(feature = "lstm")]
        if let Some(lstm) = get_best_lstm_model(str_per_lang.chars().next().unwrap() as u32) {
            let lstm_iter = LstmSegmenterIterator::new(&lstm, &str_per_lang);
            let mut r: Vec<usize> = lstm_iter.map(|n| offset + n).collect();
            result.append(&mut r);
            offset += str_per_lang.len();
            continue;
        }

        offset += str_per_lang.len();
    }
    if result.is_empty() {
        return None;
    }
    Some(result)
}

pub fn segmenter_break_utf16(input: &[u16]) -> Option<Vec<usize>> {
    let s: String = decode_utf16(input.iter().cloned())
        .map(|r| r.unwrap())
        .collect();
    let mut result: Vec<usize> = Vec::new();
    let mut offset = 0;
    for str_per_lang in LanguageIterator::new(&s) {
        if offset != 0 {
            // language break
            result.push(offset);
        }

        #[cfg(feature = "lstm")]
        if let Some(lstm) = get_best_lstm_model(str_per_lang.chars().next().unwrap() as u32) {
            let lstm_iter = LstmSegmenterIteratorUtf16::new(&lstm, &str_per_lang);
            let mut r: Vec<usize> = lstm_iter.map(|n| offset + n).collect();
            result.append(&mut r);
            offset += str_per_lang.chars().fold(0, |n, c| n + c.len_utf16());
            continue;
        }

        offset += str_per_lang.chars().fold(0, |n, c| n + c.len_utf16());
    }
    if result.is_empty() {
        return None;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "lstm")]
    use crate::complex::*;

    #[test]
    #[cfg(feature = "lstm")]
    fn thai_word_break() {
        const TEST_STR: &str = "ภาษาไทยภาษาไทย";

        let breaks = segmenter_break_utf8(TEST_STR);
        assert_eq!(breaks.unwrap(), [12, 21, 33], "Thai test");
    }

    #[test]
    #[cfg(feature = "lstm")]
    fn thai_word_break_utf16() {
        let text: [u16; 14] = [
            0x0e20, 0x0e32, 0x0e29, 0x0e32, 0x0e44, 0x0e17, 0x0e22, 0x0e20, 0x0e32, 0x0e29, 0x0e32,
            0x0e44, 0x0e17, 0x0e22,
        ];
        let breaks = segmenter_break_utf16(&text);
        assert_eq!(breaks.unwrap(), [4, 7, 11], "Thai test");

        let text: [u16; 4] = [0x0e20, 0x0e32, 0x0e29, 0x0e32];
        let breaks = segmenter_break_utf16(&text);
        assert_eq!(breaks, None, "Thai test");
    }

    #[test]
    #[cfg(feature = "lstm")]
    fn burmese_word_break() {
        // "Burmese Language" in Burmese
        const TEST_STR: &str = "မြန်မာဘာသာစကား";

        let breaks = segmenter_break_utf8(TEST_STR);
        // LSTM model breaks more characters, but it is better to return [30].
        assert_eq!(breaks.unwrap(), [12, 18, 30], "Burmese test");
    }

    #[test]
    #[cfg(feature = "lstm")]
    fn burmese_word_break_utf16() {
        // "Burmese Language" in Burmese
        let text: [u16; 14] = [
            0x1019, 0x103c, 0x1014, 0x103a, 0x1019, 0x102c, 0x1018, 0x102c, 0x101e, 0x102c, 0x1005,
            0x1000, 0x102c, 0x1038,
        ];
        let breaks = segmenter_break_utf16(&text);
        // LSTM model breaks more characters, but it is better to return [10].
        assert_eq!(breaks.unwrap(), [4, 6, 10], "Burmese utf-16 test");
    }

    #[test]
    #[cfg(feature = "lstm")]
    fn combined_word_break() {
        const TEST_STR_THAI: &str = "ภาษาไทยภาษาไทย";
        const TEST_STR_BURMESE: &str = "ဗမာနွယ်ဘာသာစကားမျာ";
        let mut sample = String::from(TEST_STR_THAI);
        sample.push_str(TEST_STR_BURMESE);

        let breaks = segmenter_break_utf8(&sample);
        assert_eq!(
            breaks.unwrap(),
            [12, 21, 33, 42, 51, 63, 75, 87],
            "Combined test"
        );
    }
}
