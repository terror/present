use crate::common::*;
use crate::{Error, Result};

const PREFIX: &str = "present";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
  program: String,
  arguments: Vec<String>,
}

impl Command {
  pub(crate) fn from(command: Vec<String>) -> Option<Self> {
    let (prefix, command) = command.split_at(1);

    if let Some(prefix) = prefix.get(0) {
      if prefix != PREFIX {
        return None;
      }
    }

    if command.is_empty() {
      return None;
    }

    let command = command
      .join(" ")
      .split(' ')
      .map(|s| s.into())
      .collect::<Vec<String>>();

    let (program, arguments) = command.split_at(1);

    program.first().map(|program| Self {
      program: program.to_string(),
      arguments: arguments.to_owned(),
    })
  }

  // TODO: Command should only get command output, not handle diffs
  pub(crate) fn execute(&self) -> Result<String> {
    let output = process::Command::new(self.program.clone())
      .args(self.arguments.clone())
      .output();

    if let Err(error) = output {
      return Err(Error::Command {
        program: self.program.clone(),
        message: error.to_string(),
      });
    }

    let output = output?;

    if !output.status.success() {
      return Err(Error::Command {
        program: self.program.clone(),
        message: String::from_utf8(output.stderr)?,
      });
    }

    Ok(String::from_utf8(output.stdout)?)
  }
}
