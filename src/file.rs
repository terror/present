use crate::common::*;

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
      commands: parser.parse()?,
    })
  }

  pub(crate) fn present(&mut self, options: RunnerOptions) -> Result {
    let diffs = self
      .commands
      .clone()
      .iter()
      .map(|command| command.execute(options.remove))
      .collect::<Result<Vec<_>, _>>()?;

    let mut offset = 0;

    for mut diff in diffs {
      diff.offset(offset);
      let prev = self.content.len_chars();
      self.content.apply(diff.clone());
      offset += self.content.len_chars() as isize - prev as isize;
    }

    match options.in_place {
      true => self.save()?,
      _ => self.print(options.pretty),
    }

    Ok(())
  }

  fn save(&self) -> Result {
    Ok(fs::write(self.path.clone(), self.content.to_string())?)
  }

  fn print(&self, pretty: bool) {
    match pretty {
      true => print_inline(&self.content.to_string()),
      _ => print!("{}", self.content),
    }
  }
}
