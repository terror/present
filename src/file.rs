use super::*;

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
      let content = codeblock.command.execute()?;

      let prefix = self
        .content
        .byte_slice(
          self.content.line_to_byte(
            self.content.byte_to_line(codeblock.position.block.start),
          )..codeblock.position.block.start,
        )
        .chars()
        .fold((String::new(), 0), |(mut prefix, column), c| {
          let width = if c == '\t' { 4 - column % 4 } else { 1 };

          if c == '>' {
            prefix.push(c);
          } else {
            if !matches!(c, ' ' | '\t') && prefix.ends_with('>') {
              prefix.push(' ');
            }

            prefix.push_str(&" ".repeat(width));
          }

          (prefix, column + width)
        })
        .0;

      let separator = if prefix.ends_with('>') { " " } else { "" };

      let content = content
        .split_inclusive('\n')
        .enumerate()
        .map(|(index, line)| {
          if self.remove && index == 0 {
            format!("{separator}{line}")
          } else {
            format!("{prefix}{separator}{line}")
          }
        })
        .collect::<String>();

      let content = if !self.remove
        && self.content.byte(codeblock.position.body.start - 1) != b'\n'
      {
        format!("\n{content}")
      } else {
        content
      };

      let content = if !self.remove
        && codeblock.position.body.end < codeblock.position.block.end
        && !content.is_empty()
        && !content.ends_with('\n')
      {
        format!("{content}\n")
      } else {
        content
      };

      Ok(Diff {
        content,
        range: match self.remove {
          // Replace the entire codeblock with `stdout`
          true => codeblock.position.block.clone(),
          // Insert in between the codeblock (start, end)
          false => codeblock.position.body.clone(),
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

    self.codeblocks = Parser::new(&self.content.to_string()).parse()?;

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn present_repeatedly() {
    #[track_caller]
    fn case(src: &str, expected: &str, remove: bool) {
      let mut file = File {
        codeblocks: Parser::new(src).parse().unwrap(),
        content: Rope::from_str(src),
        interactive: false,
        path: PathBuf::new(),
        remove,
      };

      for _ in 0..2 {
        file.present().unwrap();
        assert_eq!(file.content.to_string(), expected);
      }
    }

    let src = "foo\n\n```present echo 🚀\n```\n\n```present echo bar\nfoobarbaz\n```\n\nbaz\n";

    case(
      src,
      "foo\n\n```present echo 🚀\n🚀\n```\n\n```present echo bar\nbar\n```\n\nbaz\n",
      false,
    );

    case(src, "foo\n\n🚀\n\nbar\n\nbaz\n", true);

    case("```present printf foo", "```present printf foo\nfoo", false);
  }
}
