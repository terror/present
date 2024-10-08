use crate::{common::*, prompt, Codeblock, Diff, Parser, Result, RopeExt};

/// Represents a parsed Markdown file that can be presented
#[derive(Debug, Clone)]
pub struct File {
  codeblocks: Vec<Codeblock>,
  content: Rope,
  interactive: bool,
  path: PathBuf,
  remove: bool,
}

impl File {
  /// Create a new [`File`] by parsing the file at `path`
  ///
  /// # Errors
  ///
  /// This function will return an error if the following conditions
  /// are true:
  /// - The file is not readable into a string
  /// - The parser failed to parse the file contents
  pub fn new(path: PathBuf) -> Result<Self> {
    let content = fs::read_to_string(&path)?;

    let parser = Parser::new(&content);

    Ok(Self {
      codeblocks: parser.parse()?,
      content: Rope::from_str(&content.clone()),
      interactive: false,
      path,
      remove: false,
    })
  }

  /// Setting this to true will make the [`present`](File::present) function
  /// replace the whole code block with the command output. If kept at false
  /// (the default), it will place the output inside the code block.
  ///
  /// # Example
  ///
  /// ```ignore
  /// # use present::File;
  /// let file = File::new()
  ///     .unwrap()
  ///     .remove(true);
  /// ```
  pub fn remove(self, on: bool) -> Self {
    Self { remove: on, ..self }
  }

  /// Setting this to true will make the [`present`](File::present) function
  /// interactive. For each diff in a file, the user will be asked if they
  /// want to apply it or not.
  ///
  /// # Example
  ///
  /// ```ignore
  /// let file = File::new()
  ///     .unwrap()
  ///     .interactive(true);
  /// ```
  pub fn interactive(self, on: bool) -> Self {
    Self {
      interactive: on,
      ..self
    }
  }

  /// Returns an iterator of [`Diff`]s in the file.
  ///
  /// The [`Diff`]s are returned as results. If the command fails, the item will
  /// be of the `Err` kind.
  pub fn diffs(&self) -> impl Iterator<Item = Result<Diff>> + '_ {
    self.codeblocks.iter().map(|codeblock| {
      Ok(Diff {
        content: codeblock.command.execute()?,
        range: match self.remove {
          // Replace the entire codeblock with `stdout`
          true => {
            codeblock.position.start.start..codeblock.position.end.end + 2
          }
          // Insert in between the codeblock (start, end)
          false => {
            codeblock.position.start.end + 1..codeblock.position.end.start + 1
          }
        },
      })
    })
  }

  /// Applies all diffs produced by [`diffs`](File::diffs) by mutating self.
  ///
  /// If [`interactive`](File::interactive) is set to `true`, the user will be
  /// asked if they want to apply the change for each diff.
  pub fn present(&mut self) -> Result {
    let mut offset: isize = 0;

    let diffs = self.diffs().collect::<Result<Vec<Diff>>>()?;

    for mut diff in diffs {
      let prev = self.content.len_bytes();

      diff.offset(offset);

      if self.interactive {
        diff.print(&self.content);
        if prompt("Apply changes? [Y/N]")?.as_str() != "y" {
          continue;
        }
      }

      self.content.apply(diff.clone());
      offset += self.content.len_bytes() as isize - prev as isize;
    }

    Ok(())
  }

  /// Saves the current state to the original file.
  pub fn save(&self) -> Result {
    Ok(fs::write(&self.path, self.content.to_string())?)
  }

  /// Prints the current state to stdout. If `pretty` is true, [`termimad`] will
  /// be used to prettyprint the content.
  pub fn print(&self, pretty: bool) {
    match pretty {
      true => print_inline(&self.content.to_string()),
      _ => print!("{}", self.content),
    }
  }
}
