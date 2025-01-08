// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Similar to [`core::str::CharIndices`] for Latin-1 strings, represented as `[u8]`.
///
/// Contrary to [`core::str::CharIndices`], the second element of the
/// [`Iterator::Item`] is a [`u8`], representing a Unicode scalar value in the
/// range U+0000–U+00FF.
#[derive(Clone, Debug)]
pub struct Latin1Indices<'a> {
    front_offset: usize,
    iter: &'a [u8],
}

impl<'a> Latin1Indices<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            front_offset: 0,
            iter: input,
        }
    }

    // Update iterator with the offset
    pub(crate) fn containing(&mut self, offset: usize) -> Option<(usize, u8)> {
        self.front_offset = offset;
        self.iter
            .get(self.front_offset)
            .map(|ch| (self.front_offset, *ch))
    }

    // Update iterator by rewinding
    pub(crate) fn previous(&mut self) -> Option<(usize, u8)> {
        if self.front_offset == 0 {
            return None;
        }
        self.front_offset -= 1;
        self.iter
            .get(self.front_offset)
            .map(|ch| (self.front_offset, *ch))
    }
}

impl Iterator for Latin1Indices<'_> {
    type Item = (usize, u8);

    #[inline]
    fn next(&mut self) -> Option<(usize, u8)> {
        self.iter.get(self.front_offset).map(|ch| {
            self.front_offset += 1;
            (self.front_offset - 1, *ch)
        })
    }
}

/// Similar to [`core::str::CharIndices`] for UTF-16 strings, represented as `[u16]`.
///
/// Contrary to [`core::str::CharIndices`], the second element of the
/// [`Iterator::Item`] is a Unicode code point represented by a [`u32`],
/// rather than a Unicode scalar value represented by a [`char`], because this
/// iterator preserves unpaired surrogates.
#[derive(Clone, Debug)]
pub struct Utf16Indices<'a> {
    front_offset: usize,
    iter: &'a [u16],
}

impl<'a> Utf16Indices<'a> {
    pub fn new(input: &'a [u16]) -> Self {
        Self {
            front_offset: 0,
            iter: input,
        }
    }

    pub(crate) fn containing(&mut self, offset: usize) -> Option<(usize, u32)> {
        if let Some(ch) = self.iter.get(offset) {
            let mut ch = *ch as u32;
            if (ch & 0xfc00) == 0xd800 {
                if let Some(next) = self.iter.get(offset + 1) {
                    let next = *next as u32;
                    ch = ((ch & 0x3ff) << 10) + (next & 0x3ff) + 0x10000;
                    self.front_offset = offset + 2;
                    return Some((offset, ch));
                }
                self.front_offset = offset + 1;
                return Some((offset, ch));
            }

            if (ch & 0xfc00) == 0xdc00 {
                self.front_offset = offset + 1;
                if let Some(previous) = self.iter.get(offset - 1) {
                    let previous = *previous as u32;
                    ch = ((previous & 0x3ff) << 10) + (ch & 0x3ff) + 0x10000;
                    return Some((offset - 1, ch));
                }
                return Some((offset, ch));
            }

            self.front_offset = offset + 1;
            return Some((offset, ch));
        }

        self.front_offset = self.iter.len();
        None
    }

    pub(crate) fn previous(&mut self) -> Option<(usize, u32)> {
        let prev = self.peek_back()?;
        self.front_offset = prev.0;
        Some(prev)
        /*
                if self.front_offset == 0 {
                    return None;
                }
                self.front_offset -= 1;
                let (index, ch) = self
                    .iter
                    .get(self.front_offset)
                    .map(|ch| (self.front_offset, *ch))?;

                if (ch & 0xfc00) != 0xdc00 {
                    return Some((index, ch as u32));
                }

                if let Some(prev) = self.iter.get(self.front_offset - 1) {
                    if (prev & 0xfc00) == 0xd800 {
                        let ch = Self::surrogate_to_ucs4(*prev, ch);
                        self.front_offset -= 1;
                        return Some((index - 1, ch));
                    }
                }
                Some((index, ch as u32))
        */
    }

    pub(crate) fn peek_back(&self) -> Option<(usize, u32)> {
        if self.front_offset == 0 {
            return None;
        }
        let offset = self.front_offset - 1;
        let (index, ch) = self.iter.get(offset).map(|ch| (offset, *ch))?;

        if (ch & 0xfc00) != 0xdc00 || offset == 0 {
            return Some((offset, ch as u32));
        }

        if let Some(prev) = self.iter.get(offset - 1) {
            if (prev & 0xfc00) == 0xd800 {
                let ch = Self::surrogate_to_ucs4(*prev, ch);
                return Some((index - 1, ch));
            }
        }
        Some((index, ch as u32))
    }

    #[inline]
    fn surrogate_to_ucs4(high: u16, low: u16) -> u32 {
        (((high as u32) & 0x3ff) << 10) + ((low as u32) & 0x3ff) + 0x10000
    }
}

impl Iterator for Utf16Indices<'_> {
    type Item = (usize, u32);

    #[inline]
    fn next(&mut self) -> Option<(usize, u32)> {
        let (index, ch) = self.iter.get(self.front_offset).map(|ch| {
            self.front_offset += 1;
            (self.front_offset - 1, *ch)
        })?;

        if (ch & 0xfc00) != 0xd800 {
            return Some((index, ch as u32));
        }

        if let Some(next) = self.iter.get(self.front_offset) {
            if (next & 0xfc00) == 0xdc00 {
                // Combine low and high surrogates to UTF-32 code point.
                let ch = Self::surrogate_to_ucs4(ch, *next);
                self.front_offset += 1;
                return Some((index, ch));
            }
        }
        Some((index, ch as u32))
    }
}

#[cfg(test)]
mod tests {
    use crate::indices::*;

    #[test]
    fn latin1_indices() {
        let latin1 = [0x30, 0x31, 0x32];
        let mut indices = Latin1Indices::new(&latin1);
        let n = indices.next().unwrap();
        assert_eq!(n.0, 0);
        assert_eq!(n.1, 0x30);
        let n = indices.next().unwrap();
        assert_eq!(n.0, 1);
        assert_eq!(n.1, 0x31);
        let n = indices.next().unwrap();
        assert_eq!(n.0, 2);
        assert_eq!(n.1, 0x32);
        let n = indices.next();
        assert_eq!(n, None);
    }

    #[test]
    fn latin1_indices_containing() {
        let latin1 = [0x30, 0x31, 0x32];
        let mut indices = Latin1Indices::new(&latin1);
        let n = indices.containing(2).unwrap();
        assert_eq!(n.0, 2);
        assert_eq!(n.1, 0x32);
        let n = indices.containing(4);
        assert_eq!(n, None);
    }

    #[test]
    fn latin1_indices_previous() {
        let latin1 = [0x30, 0x31, 0x32];
        let mut indices = Latin1Indices::new(&latin1);
        indices.next();
        let n = indices.next().unwrap();
        assert_eq!(n.0, 1);
        assert_eq!(n.1, 0x31);
        let n = indices.previous().unwrap();
        assert_eq!(n.0, 1);
        assert_eq!(n.1, 0x31);
        let n = indices.previous().unwrap();
        assert_eq!(n.0, 0);
        assert_eq!(n.1, 0x30);
        let n = indices.previous();
        assert_eq!(n, None);
    }

    #[test]
    fn utf16_indices() {
        let utf16 = [0xd83d, 0xde03, 0x0020, 0xd83c, 0xdf00, 0xd800, 0x0020];
        let mut indices = Utf16Indices::new(&utf16);
        let n = indices.next().unwrap();
        assert_eq!(n.0, 0);
        assert_eq!(n.1, 0x1f603);
        let n = indices.next().unwrap();
        assert_eq!(n.0, 2);
        assert_eq!(n.1, 0x20);
        let n = indices.next().unwrap();
        assert_eq!(n.0, 3);
        assert_eq!(n.1, 0x1f300);
        // This is invalid surrogate pair.
        let n = indices.next().unwrap();
        assert_eq!(n.0, 5);
        assert_eq!(n.1, 0xd800);
        let n = indices.next().unwrap();
        assert_eq!(n.0, 6);
        assert_eq!(n.1, 0x0020);
        let n = indices.next();
        assert_eq!(n, None);
    }

    #[test]
    fn utf16_indices_containing() {
        let utf16 = [0xd83d, 0xde03, 0x0020, 0xd83c, 0xdf00, 0xd800, 0x0020];
        let mut indices = Utf16Indices::new(&utf16);
        let n = indices.containing(0).unwrap();
        assert_eq!(n.0, 0);
        assert_eq!(n.1, 0x1f603);
        let n = indices.containing(4).unwrap();
        assert_eq!(n.0, 3);
        assert_eq!(n.1, 0x1f300);
        let n = indices.next().unwrap();
        assert_eq!(n.0, 5);
        assert_eq!(n.1, 0xd800);
    }

    #[test]
    fn utf16_indices_previous() {
        let utf16 = [0xd83d, 0xde03, 0x0020, 0xd83c, 0xdf00, 0xd800, 0x0020];
        let mut indices = Utf16Indices::new(&utf16);
        indices.next();
        let n = indices.next().unwrap();
        assert_eq!(n.0, 2);
        assert_eq!(n.1, 0x20);
        let n = indices.previous().unwrap();
        assert_eq!(n.0, 2);
        assert_eq!(n.1, 0x20);
        let n = indices.previous().unwrap();
        assert_eq!(n.0, 0);
        assert_eq!(n.1, 0x1f603);
        indices.next();
        let n = indices.next().unwrap();
        assert_eq!(n.0, 2);
        assert_eq!(n.1, 0x20);
        let n = indices.next().unwrap();
        assert_eq!(n.0, 3);
        assert_eq!(n.1, 0x1f300);
        let n = indices.previous().unwrap();
        assert_eq!(n.0, 3);
        assert_eq!(n.1, 0x1f300);
        let n = indices.previous().unwrap();
        assert_eq!(n.0, 2);
        assert_eq!(n.1, 0x20);
        let n = indices.previous().unwrap();
        assert_eq!(n.0, 0);
        assert_eq!(n.1, 0x1f603);
        let n = indices.previous();
        assert_eq!(n, None);
    }
}
