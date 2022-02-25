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

    Ok(match options.in_place {
      true => self.save()?,
      _ => self.print(options.pretty),
    })
  }

  fn save(&self) -> Result {
    Ok(fs::write(self.path.clone(), self.content.to_string())?)
  }

  fn print(&self, pretty: bool) {
    if pretty {
      print_inline(&self.content.to_string());
    } else {
      print!("{}", self.content);
    }
  }
}
