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

    let mut offset: isize = 0;

    for mut diff in diffs {
      if offset < 0 {
        diff.range.start -= offset.abs() as usize;
        diff.range.end -= offset.abs() as usize;
      } else {
        diff.range.start += offset as usize;
        diff.range.end += offset as usize;
      }

      let prev = self.content.len_chars();

      self.content.apply(diff.clone());

      // Account for the increase/decrease in rope size
      if self.content.len_chars() > prev {
        offset += (self.content.len_chars() - prev) as isize;
      } else {
        offset -= (prev - self.content.len_chars()) as isize;
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
