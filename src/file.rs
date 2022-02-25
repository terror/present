use super::*;

#[derive(Debug, Clone)]
pub(crate) struct File {
  path: PathBuf,
  content: Rope,
  commands: Vec<Command>,
}

impl File {
  pub(crate) fn new(path: PathBuf) -> Result<Self> {
    let content = fs::read_to_string(&path)?;

    let parser = Parser::new(&content);

    Ok(Self {
      path,
      content: Rope::from_str(&content.clone()),
      commands: parser.commands()?,
    })
  }

  pub(crate) fn present(&mut self, options: RunnerOptions) -> Result {
    self
      .commands
      .clone()
      .iter()
      .map(|command| command.execute(options.remove))
      .collect::<Result<Vec<_>, _>>()?
      .iter()
      .for_each(|diff| self.apply_diff(diff.clone()));

    match options.in_place {
      true => fs::write(self.path.clone(), self.content.to_string())?,
      _ => print_inline(&self.content.to_string()),
    }

    Ok(())
  }

  fn apply_diff(&mut self, diff: Diff) {
    self.content.remove(diff.position.start..diff.position.end);
    self.content.insert(diff.position.start, &diff.content);
  }
}
