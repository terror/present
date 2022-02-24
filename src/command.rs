use super::*;

const PREFIX: &str = "present";

#[derive(Debug, Clone)]
pub(crate) struct Command {
  top_level: String,
  arguments: Vec<String>,
  position: Position,
}

impl Command {
  pub(crate) fn from(chunk: &Chunk) -> Option<Self> {
    let source = &chunk.src[chunk.start.start..chunk.start.end]
      .trim_start_matches(|c| c == '\n' || c == '`' || c == ' ')
      .trim_end_matches(|c| c == '\n' || c == '`' || c == ' ')
      .split(':')
      .collect::<Vec<&str>>();

    if let Some(first) = source.get(0) {
      if *first != PREFIX {
        return None;
      }
    }

    if let Some(source) = source.get(1) {
      let source = source
        .split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

      let (top_level, arguments) = source.split_at(1);

      return Some(Command {
        top_level: top_level.first().unwrap().to_string(),
        arguments: arguments.to_owned(),
        position: Position::new(chunk.start.start, chunk.end.end),
      });
    }

    None
  }

  pub(crate) fn execute(&self) -> Result<Diff> {
    let output = std::process::Command::new(self.top_level.clone())
      .args(self.arguments.clone())
      .output()?;

    if !output.status.success() {
      todo!()
    }

    let stdout = std::str::from_utf8(&output.stdout).unwrap();

    Ok(Diff {
      content: stdout.to_string(),
      position: self.position.clone(),
    })
  }
}
