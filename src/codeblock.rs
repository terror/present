use crate::{Command, Position};

#[derive(Debug, Clone)]
pub(crate) struct Codeblock {
  pub(crate) command: Command,
  pub(crate) position: Position,
}
