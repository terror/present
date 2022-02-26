use crate::common::*;

const PREFIX: &str = "present";

#[derive(Debug, Clone)]
pub(crate) struct Command {
  program: String,
  arguments: Vec<String>,
  position: Position,
}

impl Command {
  pub(crate) fn from(codeblock: Codeblock) -> Option<Self> {
    let (prefix, command) = codeblock.command.split_at(1);

    if let Some(prefix) = prefix.get(0) {
      if *prefix != PREFIX {
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

    match program.first() {
      Some(program) => Some(Command {
        program: program.to_string(),
        arguments: arguments.to_owned(),
        position: codeblock.position,
      }),
      None => None,
    }
  }

  pub(crate) fn execute(&self, remove: bool) -> Result<Diff> {
    let output = process::Command::new(self.program.clone())
      .args(self.arguments.clone())
      .output()?;

    if !output.status.success() {
      return Err(Error::CommandFailed {
        range: self.position.start.clone(),
      });
    }

    Ok(Diff {
      content: str::from_utf8(&output.stdout)?.to_string(),
      range: match remove {
        // Replace the entire codeblock with `stdout`
        true => self.position.start.start..self.position.end.end + 2,
        // Insert in between the codeblock (start, end)
        _ => self.position.start.end + 1..self.position.end.start + 1,
      },
    })
  }
}
