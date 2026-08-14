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
        arguments: Lexer::lex(&arguments.join(" ").replace("\r\n", "\n"))?,
      }),
      _ => None,
    })
  }

  pub(crate) fn execute(&self) -> Result<String> {
    #[cfg(target_os = "windows")]
    let program = std::env::var_os("PATH")
      .into_iter()
      .flat_map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
      .flat_map(|directory| {
        ["", ".com", ".exe", ".bat", ".cmd"].map(move |extension| {
          directory.join(format!("{}{extension}", self.program))
        })
      })
      .find(|path| path.is_file())
      .unwrap_or_else(|| self.program.clone().into());

    #[cfg(not(target_os = "windows"))]
    let program = &self.program;

    let output = process::Command::new(program)
      .args(&self.arguments)
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
