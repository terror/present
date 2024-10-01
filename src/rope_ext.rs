use crate::{common::*, Diff};

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
