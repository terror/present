use crate::common::*;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Position {
  pub(crate) start: Range<usize>,
  pub(crate) end: Range<usize>,
}
