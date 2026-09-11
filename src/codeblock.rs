use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Codeblock {
  pub(crate) command: Command,
  pub(crate) position: Position,
}
