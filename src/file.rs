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
    let diffs = self
      .commands
      .clone()
      .iter()
      .map(|command| command.execute(options.remove))
      .collect::<Result<Vec<_>, _>>()?;

    let mut offset = 0;

    for mut diff in diffs {
      diff.range.start += offset;
      diff.range.end += offset;

      let prev = self.content.len_chars();

      self.content.apply(diff.clone());

      // Account for the increase in rope size
      if self.content.len_chars() > prev {
        offset += self.content.len_chars() - prev;
      }
    }

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
