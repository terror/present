use super::*;

#[derive(Debug, Clone)]
pub(crate) struct Diff {
  pub(crate) content: String,
  pub(crate) range: Range<usize>,
}
