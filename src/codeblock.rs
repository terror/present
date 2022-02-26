use crate::common::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Codeblock {
  pub(crate) command: Vec<String>,
  pub(crate) position: Position,
}

impl Codeblock {
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
      position: Position {
        start: start_start..start_end,
        end: end_start..end_end,
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple() {
    let codeblock = Codeblock::new("```present echo bar\n```", 0..22);

    assert_eq!(codeblock.command, vec!["present", "echo", "bar"]);

    assert_eq!(
      codeblock.position,
      Position {
        start: 0..19,
        end: 19..21
      }
    );
  }

  #[test]
  fn with_exterior_content() {
    let codeblock =
      Codeblock::new("foo\n\n```present echo bar\n```\n\nbaz", 5..29);

    assert_eq!(codeblock.command, vec!["present", "echo", "bar"]);

    assert_eq!(
      codeblock.position,
      Position {
        start: 5..24,
        end: 24..28
      }
    );
  }
}
