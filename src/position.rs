use crate::common::*;

#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct Position {
  pub(crate) start: Range<usize>,
  pub(crate) end: Range<usize>,
}
