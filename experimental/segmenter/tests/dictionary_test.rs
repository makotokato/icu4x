// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;
use icu_segmenter::provider::*;
use icu_segmenter::DictionarySegmenter;
use zerovec::ZeroVec;

#[test]
fn cj_trie_iter_test() {
    const CJ_DICTIONARY: &[u8; 2003390] = include_bytes!("testdata/cjdic.dict.bin");
    let trie_data =
        unsafe { core::mem::transmute::<&[u8; 2003390], &[u16; 1001695]>(CJ_DICTIONARY) };
    let data = UCharDictionaryBreakDataV1 {
        trie_data: ZeroVec::from_slice(trie_data),
    };
    let payload = DataPayload::<UCharDictionaryBreakDataV1Marker>::from_owned(data);
    let segmenter = DictionarySegmenter::try_new(payload).expect("Data exists");

    // Match case
    let s = "龟山岛";
    let mut iterator = segmenter.segment_str(&s);
    assert_eq!(iterator.next(), Some(9));
    assert_eq!(iterator.next(), None);

    let s_utf16: Vec<u16> = s.encode_utf16().collect();
    let mut iterator = segmenter.segment_utf16(&s_utf16);
    assert_eq!(iterator.next(), Some(3));
    assert_eq!(iterator.next(), None);

    // Match case, then no match case
    let s = "エディターエディ";
    let mut iterator = segmenter.segment_str(&s);
    assert_eq!(iterator.next(), Some(15));
    assert_eq!(iterator.next(), Some(24));
    assert_eq!(iterator.next(), None);

    let s_utf16: Vec<u16> = s.encode_utf16().collect();
    let mut iterator = segmenter.segment_utf16(&s_utf16);
    assert_eq!(iterator.next(), Some(5));
    assert_eq!(iterator.next(), Some(8));
    assert_eq!(iterator.next(), None);
}

#[test]
fn khmer_trie_iter_test() {
    const KHMER_DICTIONARY: &[u8; 798374] = include_bytes!("testdata/km.dict.bin");
    let trie_data =
        unsafe { core::mem::transmute::<&[u8; 798374], &[u16; (798374 / 2)]>(KHMER_DICTIONARY) };
    let data = UCharDictionaryBreakDataV1 {
        trie_data: ZeroVec::from_slice(trie_data),
    };
    let payload = DataPayload::<UCharDictionaryBreakDataV1Marker>::from_owned(data);
    let segmenter = DictionarySegmenter::try_new(payload).expect("Data exists");
    let s = "ភាសាខ្មែរភាសាខ្មែរភាសាខ្មែរ";
    let result: Vec<usize> = segmenter.segment_str(&s).collect();
    assert_eq!(result, vec![27, 54, 81]);

    let s_utf16: Vec<u16> = s.encode_utf16().collect();
    let result: Vec<usize> = segmenter.segment_utf16(&s_utf16).collect();
    assert_eq!(result, vec![9, 18, 27]);
}

#[test]
fn lao_trie_iter_test() {
    const LAO_DICTIONARY: &[u8; 292456] = include_bytes!("testdata/lo.dict.bin");
    let trie_data =
        unsafe { core::mem::transmute::<&[u8; 292456], &[u16; (292456 / 2)]>(LAO_DICTIONARY) };
    let data = UCharDictionaryBreakDataV1 {
        trie_data: ZeroVec::from_slice(trie_data),
    };
    let payload = DataPayload::<UCharDictionaryBreakDataV1Marker>::from_owned(data);
    let segmenter = DictionarySegmenter::try_new(payload).expect("Data exists");
    let s = "ພາສາລາວພາສາລາວພາສາລາວ";
    let r: Vec<usize> = segmenter.segment_str(&s).collect();
    assert_eq!(r, vec![12, 21, 33, 42, 54, 63]);

    let s_utf16: Vec<u16> = s.encode_utf16().collect();
    let r: Vec<usize> = segmenter.segment_utf16(&s_utf16).collect();
    assert_eq!(r, vec![4, 7, 11, 14, 18, 21]);
}
