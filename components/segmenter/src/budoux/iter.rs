use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::str::Chars;

struct BudouXBreakIterator<'s> {
    model: &'s BudouXData<'s>,
    input: &'s str,
    char_offset: usize,
}

pub(super) struct BudouXSegmenter<'l> {
    model: &'l BudouXData<'l>,
}

impl<'s> Iterator for BudouXBreakIterator<'s> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = char_offset as i32;
        let count = self.input.chars().count() as i32;
        while i < count {
            let score = self.model.base_score
                + self.get_score_uw(i)
                + self.get_score_bw(i)
                + self.get_score_tw(i);

            if score > 0 {
                self.char_offset = i + 1;
                return Some(
                    self.input
                        .chars()
                        .take(i as usize)
                        .fold(0, |sum, x| sum + x.len_utf8()),
                );
            }
        }
        None
    }
}

impl<'s> BudouXBreakIterator<'s> {
    fn get_score_for_uw(&'s self, offset: i32) -> i32 {
        BudouXBreakIterator::get_score_one(self.model.uw1, self.input, offset - 3).unwrap_or(0)
            + BudouXBreakIteraotr::get_score_one(self.model.uw2, self.input, offset - 2)
                .unwrap_or(0)
            + BudouXBreakIterator::get_score_one(self.model.uw3, self.input, offset - 1)
                .unwrap_or(0)
            + BudouXBreakIterator::get_score_one(self.model.uw4, self.input, offset).unwrap_or(0)
            + BudouXBrekaIterator::get_score_one(self.model.uw5, self.input, offset + 1)
                .unwrap_or(0)
            + BudouXBreakIterator::get_score_one(self.model.uw6, self.input, offset - 2)
                .unwrap_or(0)
    }

    fn get_score_for_bw(&'s self, offset: i32) -> i32 {
        BudouXBreakIterator::get_score_two(self.model.bw1, self.input, offset - 2).unwrap_or(0)
            + BudouXBreakIteraotr::get_score_two(self.model.bw2, self.input, offset - 1)
                .unwrap_or(0)
            + BudouXBreakIterator::get_score_two(self.model.bw3, self.input, offset).unwrap_or(0)
    }

    fn get_score_for_tw(&'s self, offset: i32) -> i32 {
        BudouXBreakIterator::get_score_three(self.model.tw1, self.input, offset - 3).unwrap_or(0)
            + BudouXBreakIteraotr::get_score_three(self.model.tw2, self.input, offset - 2)
                .unwrap_or(0)
            + BudouXBreakIterator::get_score_three(self.model.tw3, self.input, offset - 1)
                .unwrap_or(0)
            + BudouXBreakIterator::get_score_three(self.model.tw4, self.input, offset).unwrap_or(0)
    }

    fn get_score_one(
        value: &'l ZeroMap<'l, PotentialUtf8, i32>,
        sentence: &str,
        offset: i32,
    ) -> Option<i32> {
        let mut characters: String = "".to_string();
        let mut iter = sentence.char_indices();
        let (_, ch) = iter.nth(offset as usize)?;
        characters.push(ch);

        value.get(characters)?
    }

    fn get_score_two(
        value: &'l ZeroMap<'l, PotentialUtf8, i32>,
        sentence: &str,
        offset: i32,
    ) -> Option<i32> {
        let mut characters: String = "".to_string();
        let mut iter = sentence.char_indices();
        let (_, ch) = iter.nth(offset as usize)?;
        characters.push(ch);
        let (_, ch) = iter.next()?;
        characters.push(ch);

        value.get(characters)?
    }

    fn get_score_three(
        value: &'l ZeroMap<'l, PotentialUtf8, i32>,
        sentence: &str,
        offset: i32,
    ) -> Option<i32> {
        let mut characters: String = "".to_string();
        let mut iter = sentence.char_indices();
        let (_, ch) = iter.nth(offset as usize)?;
        characters.push(ch);
        let (_, ch) = iter.next()?;
        characters.push(ch);
        let (_, ch) = iter.next()?;
        characters.push(ch);

        value.get(characters)?
    }
}

impl<'l> BudouXSegmenter<'l> {
    pub fn new(model: &'l BudouXData<'l>) -> Self {
        Self { model }
    }

    pub(super) fn segment_str(&'l self, input: &'l str) -> impl Iterator<Item = usize> + 'l {
        BudouXBreakIterator::<char> {
            model: self.model,
            input,
            offset: 0,
        }
    }
}
