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
      commands: parser.extract_commands()?,
    })
  }

  pub(crate) fn apply_edit(&mut self) -> Result {
    for command in self.commands.clone() {
      let result = command.execute()?;

      self
        .content
        .remove(command.position.start..command.position.end);

      self.content.insert(command.position.start, &result);
    }

    fs::write(self.path.clone(), self.content.to_string())?;

    Ok(())
  }
}
