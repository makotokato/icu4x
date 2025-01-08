// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_segmenter::{
    GraphemeClusterSegmenter, SentenceBreakInvariantOptions, SentenceSegmenter,
    WordBreakInvariantOptions, WordSegmenter,
};

#[test]
fn preceding_word_str() {
    let segmenter = WordSegmenter::new_auto(WordBreakInvariantOptions::default());
    let mut iter = segmenter.segment_str("hello world 123");

    assert_eq!(iter.preceding(0), Some(0), "First segment");
    assert_eq!(iter.is_word_like(), false, "SOT isn't word like");

    assert_eq!(iter.next(), Some(5), "Second segment");
    assert_eq!(iter.is_word_like(), true, "Letter isn't word like");

    assert_eq!(iter.preceding(7), Some(6), "after space");
    assert_eq!(iter.is_word_like(), false, "Space is word like");

    assert_eq!(iter.preceding(0), None, "offset is smaller that current");
    assert_eq!(iter.preceding(5), None, "offset is smaller that current");
}

#[test]
fn containing_word_utf16() {
    let segmenter = WordSegmenter::new_auto(WordBreakInvariantOptions::default());
    let utf16_en: Vec<u16> = "hello world 123".encode_utf16().collect();
    let mut iter = segmenter.segment_utf16(&utf16_en);

    assert_eq!(iter.preceding(0), Some(0), "First segment");
    assert_eq!(iter.is_word_like(), false, "SOT is word like");

    assert_eq!(iter.next(), Some(5), "Second segment");
    assert_eq!(iter.is_word_like(), true, "Letter isn't word like");

    assert_eq!(iter.preceding(9), Some(6), "after space");
    assert_eq!(iter.is_word_like(), false, "Space is word like");

    assert_eq!(iter.preceding(0), None, "offset is smaller that current");
}

#[test]
fn containing_sentence_str() {
    let segmenter = SentenceSegmenter::new(SentenceBreakInvariantOptions::default());
    let mut iter = segmenter.segment_str("hello world. ようこそ");

    assert_eq!(iter.preceding(0), Some(0), "SOT");
    assert_eq!(iter.preceding(15), Some(13), "Second segment");
}

#[test]
fn containing_sentence_utf16() {
    let segmenter = SentenceSegmenter::new(SentenceBreakInvariantOptions::default());
    let utf16_en: Vec<u16> = "hello world. ようこそ".encode_utf16().collect();
    let mut iter = segmenter.segment_utf16(&utf16_en);

    assert_eq!(iter.preceding(0), Some(0), "SOT");
    assert_eq!(iter.preceding(15), Some(13), "Second segment");
}

#[test]
fn containing_grapheme_cluster_utf16() {
    let segmenter = GraphemeClusterSegmenter::new();
    // This emoji is USA national flag
    let utf16_en: Vec<u16> = "hello world. \u{1f1fa}\u{1f1f2}".encode_utf16().collect();
    let mut iter = segmenter.segment_utf16(&utf16_en);

    assert_eq!(iter.preceding(0), Some(0), "SOT");
    assert_eq!(
        iter.preceding(15),
        Some(13),
        "Start of Emoji grapheme cluster"
    );
    assert_eq!(
        iter.preceding(16),
        Some(13),
        "Start of Emoji grapheme cluster"
    );
}
