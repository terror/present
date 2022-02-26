use crate::common::*;

#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct Codeblock {
  pub(crate) start: Range<usize>,
  pub(crate) end: Range<usize>,
}

impl Codeblock {
  pub(crate) fn new(start: Range<usize>, end: Range<usize>) -> Self {
    Self { start, end }
  }
}
