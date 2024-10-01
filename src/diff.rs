use crate::{common::*, RopeExt};

/// Represents a diff in a [`File`](crate::File)
#[derive(Debug, Clone)]
pub struct Diff {
  /// A string that will be inserted into `range`
  pub content: String,
  /// A range from the original string that will get replaced
  pub range: Range<usize>,
}

impl Diff {
  /// Adjusts the diff's range by the given offset.
  ///
  /// This method modifies the start and end points of the diff's range
  /// based on the provided offset. It handles both positive and negative
  /// offsets, using saturating arithmetic to prevent underflow or overflow.
  pub(crate) fn offset(&mut self, offset: isize) {
    if offset >= 0 {
      let offset = offset as usize;
      self.range.start = self.range.start.saturating_add(offset);
      self.range.end = self.range.end.saturating_add(offset);
    } else {
      let abs_offset = offset.unsigned_abs();
      self.range.start = self.range.start.saturating_sub(abs_offset);
      self.range.end = self.range.end.saturating_sub(abs_offset);
    }
  }

  /// Prints the diff by using [`TextDiff`].
  ///
  /// Since the struct does not store any context of what it's diffing on, you
  /// need to supply the original content (as a [`Rope`] reference) to this
  /// function.
  pub fn print(&self, content: &Rope) {
    for change in TextDiff::from_lines(
      &content.to_string(),
      &content.simulate(self.clone()).to_string(),
    )
    .iter_all_changes()
    {
      let (sign, style) = match change.tag() {
        ChangeTag::Delete => ("-", Style::new().red()),
        ChangeTag::Insert => ("+", Style::new().green()),
        ChangeTag::Equal => ("", Style::new()),
      };
      eprint!("{}{}", style.apply_to(sign).bold(), style.apply_to(change));
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

  #[test]
  fn offset_positive_large() {
    let mut diff = diff();

    diff.offset(isize::MAX);

    assert_eq!(
      diff.range,
      (1 + isize::MAX as usize)..(4 + isize::MAX as usize)
    );
  }

  #[test]
  fn offset_negative_large() {
    let mut diff = diff();
    diff.offset(isize::MIN);
    assert_eq!(diff.range, 0..0);
  }
}
