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
      .for_each(|diff| self.content.apply(diff.clone()));

    self.save(options.in_place)
  }

  fn save(&self, in_place: bool) -> Result {
    Ok(match in_place {
      true => fs::write(self.path.clone(), self.content.to_string())?,
      _ => print_inline(&self.content.to_string()),
    })
  }
}
