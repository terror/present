use super::*;

pub(crate) trait RopeExt {
  fn apply(&mut self, diff: Diff);
}

impl RopeExt for Rope {
  fn apply(&mut self, diff: Diff) {
    self.remove(diff.position.start..diff.position.end);
    self.insert(diff.position.start, &diff.content);
  }
}
