use super::*;

const PREFIX: &str = "present";

#[derive(Debug, Clone)]
pub(crate) struct Command {
  program: String,
  arguments: Vec<String>,
  codeblock: Codeblock,
}

impl Command {
  pub(crate) fn from(chunk: Chunk) -> Option<Self> {
    let (prefix, command) = chunk.command.split_at(1);

    if let Some(prefix) = prefix.get(0) {
      if *prefix != PREFIX {
        return None;
      }
    }

    if !command.is_empty() {
      let command = command
        .join(" ")
        .split(' ')
        .map(|s| s.into())
        .collect::<Vec<String>>();

      let (program, arguments) = command.split_at(1);

      if let Some(program) = program.first() {
        return Some(Command {
          program: program.to_string(),
          arguments: arguments.to_owned(),
          codeblock: chunk.codeblock,
        });
      }
    }

    None
  }

  pub(crate) fn execute(&self, remove: bool) -> Result<Diff> {
    let output = process::Command::new(self.program.clone())
      .args(self.arguments.clone())
      .output()?;

    if !output.status.success() {
      return Err(Error::CommandFailed {
        range: self.codeblock.start.clone(),
      });
    }

    let stdout = str::from_utf8(&output.stdout)?;

    let range = match remove {
      // Replace the entire codeblock with `stdout`
      true => self.codeblock.start.start..self.codeblock.end.end + 2,
      // Insert in between the codeblock (start, end)
      _ => self.codeblock.start.end + 1..self.codeblock.end.start + 1,
    };

    Ok(Diff {
      content: stdout.to_string(),
      range,
    })
  }
}
