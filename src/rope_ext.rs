use crate::{common::*, Diff};

pub(crate) trait RopeExt {
  fn apply(&mut self, diff: Diff);
  fn simulate(&self, diff: Diff) -> Rope;
}

impl RopeExt for Rope {
  fn apply(&mut self, diff: Diff) {
    self.remove(diff.range.clone());
    self.insert(diff.range.start, &diff.content);
  }

  fn simulate(&self, diff: Diff) -> Rope {
    let mut clone = self.clone();
    clone.remove(diff.range.clone());
    clone.insert(diff.range.start, &diff.content);
    clone
  }
}
