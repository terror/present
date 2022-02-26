use super::*;

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

  struct Test {
    content: String,
    expected_range: Range<usize>,
    offset: isize,
    range: Range<usize>,
  }

  impl Test {
    fn new() -> Self {
      Self {
        content: String::new(),
        expected_range: Range::default(),
        offset: 0,
        range: Range::default(),
      }
    }

    fn content(self, content: &str) -> Self {
      Self {
        content: content.to_string(),
        ..self
      }
    }

    fn expected_range(self, expected_range: Range<usize>) -> Self {
      Self {
        expected_range,
        ..self
      }
    }

    fn offset(self, offset: isize) -> Self {
      Self { offset, ..self }
    }

    fn range(self, range: Range<usize>) -> Self {
      Self { range, ..self }
    }

    fn run(self) {
      let mut diff = Diff {
        content: self.content,
        range: self.range,
      };

      diff.offset(self.offset);

      assert_eq!(diff.range, self.expected_range);
    }
  }

  #[test]
  fn offset_positive() {
    Test::new()
      .content("foobar")
      .range(1..4)
      .offset(1)
      .expected_range(2..5)
      .run();
  }

  #[test]
  fn offset_negative() {
    Test::new()
      .content("foobar")
      .range(1..4)
      .offset(-1)
      .expected_range(0..3)
      .run();
  }

  #[test]
  fn offset_negative_below_zero() {
    Test::new()
      .content("foobar")
      .range(0..3)
      .offset(-1)
      .expected_range(0..2)
      .run();
  }
}
