use crate::common::*;

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
