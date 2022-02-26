use crate::common::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Chunk {
  pub(crate) command: Vec<String>,
  pub(crate) codeblock: Codeblock,
}

impl Chunk {
  pub(crate) fn new(src: &'_ str, range: Range<usize>) -> Self {
    let start_start = range.start;
    let mut start_end = start_start;

    while let Some(ch) = src.chars().nth(start_end) {
      match ch {
        '`' => start_end += 1,
        _ => break,
      }
    }

    while let Some(ch) = src.chars().nth(start_end) {
      match ch {
        '`' | '\n' => break,
        _ => start_end += 1,
      }
    }

    let end_end = range.end - 1;
    let mut end_start = end_end;

    while let Some(ch) = src.chars().nth(end_start) {
      match ch {
        '`' => break,
        _ => end_start -= 1,
      }
    }

    while let Some(ch) = src.chars().nth(end_start) {
      match ch {
        '`' => end_start -= 1,
        _ => break,
      }
    }

    Self {
      command: src[start_start..start_end]
        .trim_start_matches('`')
        .split(' ')
        .map(|s| s.into())
        .collect::<Vec<String>>(),
      codeblock: Codeblock::new(start_start..start_end, end_start..end_end),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use unindent::Unindent;

  struct Test {
    source: String,
    range: Range<usize>,
    command: Vec<String>,
    codeblock: Codeblock,
  }

  impl Test {
    fn new() -> Self {
      Self {
        source: String::new(),
        range: Range::default(),
        command: Vec::new(),
        codeblock: Codeblock::default(),
      }
    }

    fn source(self, source: &str) -> Self {
      Self {
        source: source.unindent(),
        ..self
      }
    }

    fn range(self, range: Range<usize>) -> Self {
      Self { range, ..self }
    }

    fn command(self, command: Vec<&str>) -> Self {
      Self {
        command: command.iter().map(|s| s.to_string()).collect(),
        ..self
      }
    }

    fn codeblock(self, start: Range<usize>, end: Range<usize>) -> Self {
      Self {
        codeblock: Codeblock::new(start, end),
        ..self
      }
    }

    fn run(self) -> Result {
      let chunk = Chunk::new(&self.source, self.range);

      assert_eq!(chunk.command, self.command);
      assert_eq!(chunk.codeblock, self.codeblock);

      Ok(())
    }
  }

  #[test]
  fn simple() -> Result {
    Test::new()
      .source(
        "
        ```present echo bar
        ```
        ",
      )
      .range(0..24)
      .command(vec!["present", "echo", "bar"])
      .codeblock(0..19, 19..23)
      .run()
  }

  #[test]
  fn with_exterior_content() -> Result {
    Test::new()
      .source(
        "
        foo

        ```present echo bar
        ```

        baz
        ",
      )
      .range(5..29)
      .command(vec!["present", "echo", "bar"])
      .codeblock(5..24, 24..28)
      .run()
  }
}
