use crate::common::*;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Position {
  pub(crate) block: Range<usize>,
  pub(crate) body: Range<usize>,
}
