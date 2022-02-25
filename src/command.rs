use super::*;

const PREFIX: &str = "present";

#[derive(Debug, Clone)]
pub(crate) struct Command {
  top_level: String,
  arguments: Vec<String>,
  start: Position,
  end: Position,
}

impl Command {
  pub(crate) fn from(chunk: &Chunk) -> Option<Self> {
    let trim = |c: char| c == '\n' || c == '`' || c == ' ';

    let source = &chunk.src[chunk.start.start..chunk.start.end]
      .trim_start_matches(trim)
      .trim_end_matches(trim)
      .split(' ')
      .collect::<Vec<&str>>()
      .iter()
      .map(|s| s.to_string())
      .collect::<Vec<String>>();

    let (first, rest) = source.split_at(1);

    if let Some(first) = first.get(0) {
      if *first != PREFIX {
        return None;
      }
    }

    if !rest.is_empty() {
      let source = rest
        .join(" ")
        .split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

      let (top_level, arguments) = source.split_at(1);

      return Some(Command {
        top_level: top_level.first().unwrap().to_string(),
        arguments: arguments.to_owned(),
        start: chunk.start.clone(),
        end: chunk.end.clone(),
      });
    }

    None
  }

  pub(crate) fn execute(&self, remove: bool) -> Result<Diff> {
    let output = process::Command::new(self.top_level.clone())
      .args(self.arguments.clone())
      .output()?;

    if !output.status.success() {
      return Err(Error::MalformedCommand {
        position: self.start.clone(),
      });
    }

    let stdout = str::from_utf8(&output.stdout)?;

    let position = match remove {
      true => Position::new(self.start.start, self.end.end),
      _ => Position::new(self.start.end, self.end.start),
    };

    Ok(Diff {
      content: stdout.to_string(),
      position,
    })
  }
}
