use crate::{common::*, Error, Lexer, Result};

const PREFIX: &str = "present";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Command {
  program: String,
  arguments: Vec<String>,
}

impl Command {
  pub(crate) fn from(command: Vec<String>) -> Result<Option<Self>> {
    Ok(match &*command {
      [prefix, program, arguments @ ..] if prefix == PREFIX => Some(Self {
        program: program.to_string(),
        arguments: Lexer::lex(&arguments.join(" "))?,
      }),
      _ => None,
    })
  }

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
