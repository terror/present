use crate::common::*;
use crate::RopeExt;

/// Represents a diff in a [`File`](crate::File)
#[derive(Debug, Clone)]
pub struct Diff {
  /// A string that will be inserted into `range`
  pub content: String,
  /// A range from the original string that will get replaced
  pub range: Range<usize>,
}

impl Diff {
  pub(crate) fn offset(&mut self, offset: isize) {
    if offset < 0 {
      self.range.start = self.range.start.saturating_sub(offset.unsigned_abs());
      self.range.end = self.range.end.saturating_sub(offset.unsigned_abs());
    } else {
      self.range.start += offset as usize;
      self.range.end += offset as usize;
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
}
