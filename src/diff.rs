use crate::common::*;

#[derive(Debug, Clone)]
pub(crate) struct Diff {
  pub(crate) content: String,
  pub(crate) range: Range<usize>,
}

impl Diff {
  pub(crate) fn offset(&mut self, offset: isize) {
    if offset < 0 {
      self.range.start = self.range.start.saturating_sub(offset.abs() as usize);
      self.range.end = self.range.end.saturating_sub(offset.abs() as usize);
    } else {
      self.range.start += offset as usize;
      self.range.end += offset as usize;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn diff() -> Diff {
    Diff {
      content: "foobar".into(),
      range: 1..4,
    }
  }

  #[test]
  fn offset_positive() {
    let mut diff = diff();
    diff.offset(1);
    assert_eq!(diff.range, 2..5);
  }

  #[test]
  fn offset_negative() {
    let mut diff = diff();
    diff.offset(-1);
    assert_eq!(diff.range, 0..3);
  }

  #[test]
  fn offset_negative_overflow() {
    let mut diff = diff();
    diff.offset(-10);
    assert_eq!(diff.range, 0..0);
  }
}
