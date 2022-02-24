use super::*;

const PREFIX: &str = "present";

#[derive(Debug, Clone)]
pub(crate) struct Command {
  top_level: String,
  arguments: Vec<String>,
  pub(crate) position: Position,
}

impl Command {
  fn new() -> Self {
    Self {
      top_level: String::new(),
      arguments: Vec::new(),
      position: Position::default(),
    }
  }

  pub(crate) fn set_top_level(self, top_level: &str) -> Self {
    Self {
      top_level: top_level.to_owned(),
      ..self
    }
  }

  pub(crate) fn set_arguments(self, arguments: Vec<&str>) -> Self {
    Self {
      arguments: arguments
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>(),
      ..self
    }
  }

  pub(crate) fn set_position(self, position: Position) -> Self {
    Self { position, ..self }
  }

  pub(crate) fn parse(source: &str) -> Option<Self> {
    let command = Command::new();

    let source = source
      .trim_start_matches('`')
      .trim_end_matches('`')
      .trim_end_matches('\n')
      .split(':')
      .collect::<Vec<&str>>();

    // Commands must start with the prefix `present`.
    if let Some(first) = source.get(0) {
      if *first != PREFIX {
        return None;
      }
    }

    if let Some(source) = source.get(1) {
      let source = source.split(' ').collect::<Vec<&str>>();

      let (top_level, arguments) = source.split_at(1);

      return Some(
        command
          .set_top_level(top_level[0])
          .set_arguments(arguments.to_owned()),
      );
    }

    None
  }

  pub(crate) fn execute(&self) -> Result<String> {
    let output = std::process::Command::new(self.top_level.clone())
      .args(self.arguments.clone())
      .output()?;

    if !output.status.success() {
      todo!()
    }

    let stdout = std::str::from_utf8(&output.stdout).unwrap();

    Ok(stdout.to_string())
  }
}
