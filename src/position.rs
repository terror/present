use super::*;

#[derive(Debug, Default, Clone)]
pub struct Position {
  pub(crate) start: usize,
  pub(crate) end: usize,
}

impl Display for Position {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.start, self.end)
  }
}

impl From<Range<usize>> for Position {
  fn from(range: Range<usize>) -> Self {
    Self {
      start: range.start,
      end: range.end,
    }
  }
}

impl Position {
  pub(crate) fn new(start: usize, end: usize) -> Self {
    Self { start, end }
  }
}
