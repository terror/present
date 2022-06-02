use crate::common::*;
use crate::{prompt, Command, Diff, Parser, Position, Result, RopeExt};

#[derive(Debug, Clone)]
pub struct File {
  path: PathBuf,
  content: Rope,
  commands: Vec<(Position, Command)>,
  remove: bool,
}

impl File {
  pub fn new(path: PathBuf) -> Result<Self> {
    let content = fs::read_to_string(&path)?;

    let parser = Parser::new(&content);

    Ok(Self {
      path,
      content: Rope::from_str(&content.clone()),
      commands: parser.parse()?,
      remove: false,
    })
  }

  pub fn remove(self, on: bool) -> Self {
    Self { remove: on, ..self }
  }

  pub fn diffs(&self) -> impl Iterator<Item = Result<Diff>> + '_ {
    self.commands.iter().map(|(position, command)| {
      Ok(Diff {
        content: command.execute()?,
        range: match self.remove {
          // Replace the entire codeblock with `stdout`
          true => position.start.start..position.end.end + 2,
          // Insert in between the codeblock (start, end)
          false => position.start.end + 1..position.end.start + 1,
        },
      })
    })
  }

  pub fn present(&mut self) -> Result {
    let mut offset = 0;

    let diffs = self.diffs().collect::<Result<Vec<Diff>>>()?;
    for mut diff in diffs {
      let prev = self.content.len_chars();
      diff.offset(offset);

      self.content.apply(diff.clone());
      offset += self.content.len_chars() as isize - prev as isize;
    }

    Ok(())
  }

  pub fn present_interactive(&mut self) -> Result {
    let mut offset = 0;

    let diffs = self.diffs().collect::<Result<Vec<Diff>>>()?;
    for mut diff in diffs {
      let prev = self.content.len_chars();
      diff.offset(offset);

      diff.print(&self.content);
      if prompt("Apply changes? [Y/N]")?.as_str() == "y" {
        self.content.apply(diff.clone());
        offset += self.content.len_chars() as isize - prev as isize;
      }
    }

    Ok(())
  }

  pub fn save(&self) -> Result {
    Ok(fs::write(self.path.clone(), self.content.to_string())?)
  }

  pub fn print(&self, pretty: bool) {
    match pretty {
      true => print_inline(&self.content.to_string()),
      _ => print!("{}", self.content),
    }
  }
}
