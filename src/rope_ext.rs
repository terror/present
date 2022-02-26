use crate::common::*;

pub(crate) trait RopeExt {
  fn apply(&mut self, diff: Diff);
}

impl RopeExt for Rope {
  fn apply(&mut self, diff: Diff) {
    self.remove(diff.range.clone());
    self.insert(diff.range.start, &diff.content);
  }
}
