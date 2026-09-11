use crate::{common::*, Error, Lexer, Result};

const PREFIX: &str = "present";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Command {
  program: String,
  arguments: Vec<String>,
}

impl Command {
  pub(crate) fn from(command: &str) -> Result<Option<Self>> {
    let Some((PREFIX, command)) = command
      .trim_start_matches([' ', '\t', '\r'])
      .split_once([' ', '\t', '\r'])
    else {
      return Ok(None);
    };

    let mut command = Lexer::lex(command)?.into_iter();

    Ok(command.next().map(|program| Self {
      program,
      arguments: command.collect(),
    }))
  }

  pub(crate) fn execute(&self) -> Result<String> {
    #[cfg(target_os = "windows")]
    let program = &Self::resolve(&self.program);

    #[cfg(not(target_os = "windows"))]
    let program = &self.program;

    let output = process::Command::new(program)
      .args(&self.arguments)
      .output();

    #[cfg(target_os = "windows")]
    let output = match output {
      Err(error) if error.raw_os_error() == Some(193) => {
        process::Command::new(Self::resolve("bash"))
          .args(["-c", "exec \"$@\"", "--"])
          .arg(program)
          .args(&self.arguments)
          .output()
      }
      output => output,
    };

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

  #[cfg(target_os = "windows")]
  fn resolve(program: &str) -> PathBuf {
    std::env::var_os("PATH")
      .into_iter()
      .flat_map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
      .flat_map(|directory| {
        ["", ".com", ".exe", ".bat", ".cmd"]
          .map(move |extension| directory.join(format!("{program}{extension}")))
      })
      .find(|path| path.is_file())
      .unwrap_or_else(|| program.into())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse() {
    #[track_caller]
    fn case(src: &str, program: &str, arguments: &[&str]) {
      assert_eq!(
        Command::from(src).unwrap(),
        Some(Command {
          program: program.into(),
          arguments: arguments
            .iter()
            .map(|argument| (*argument).into())
            .collect(),
        }),
      );
    }

    case("present foo", "foo", &[]);
    case(r#"present "foo bar" baz\ qux"#, "foo bar", &["baz qux"]);
    case(r#"present 'foo bar' baz"#, "foo bar", &["baz"]);
    case(r#"present foo\ bar baz"#, "foo bar", &["baz"]);

    case(
      r#"present "foo\n\t\r\q\\bar" baz"#,
      "foo\n\t\r\\q\\bar",
      &["baz"],
    );
  }
}
