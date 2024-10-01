use crate::{common::*, Diff};
use unicode_segmentation::UnicodeSegmentation;

pub(crate) trait RopeExt {
  fn apply(&mut self, diff: Diff);
  fn simulate(&self, diff: Diff) -> Rope;
}

impl RopeExt for Rope {
  fn apply(&mut self, diff: Diff) {
    let start = self.byte_to_char(diff.range.start);
    let end = self.byte_to_char(diff.range.end);
    self.remove(start..end);
    self.insert(start, &diff.content);
  }

  fn simulate(&self, diff: Diff) -> Rope {
    let mut clone = self.clone();
    let start = clone.byte_to_char(diff.range.start);
    let end = clone.byte_to_char(diff.range.end);
    clone.remove(start..end);
    clone.insert(start, &diff.content);
    clone
  }
}

pub(crate) fn byte_index_to_grapheme_index(
  s: &str,
  byte_index: usize,
) -> usize {
  s.grapheme_indices(true)
    .take_while(|(i, _)| *i < byte_index)
    .count()
}

pub(crate) fn grapheme_index_to_byte_index(
  s: &str,
  grapheme_index: usize,
) -> usize {
  s.grapheme_indices(true)
    .nth(grapheme_index)
    .map(|(i, _)| i)
    .unwrap_or(s.len())
}
