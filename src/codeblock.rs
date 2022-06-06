use crate::{common::*, Command, Position, Result};

pub(crate) fn parse_codeblock(
  src: &'_ str,
  range: Range<usize>,
) -> Result<Option<(Position, Command)>> {
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

  let codeblock_args = src[start_start..start_end]
    .trim_start_matches('`')
    .split(' ')
    .map(|s| s.into())
    .collect::<Vec<String>>();

  Ok(match Command::from(codeblock_args)? {
    Some(command) => {
      let position = Position {
        start: start_start..start_end,
        end: end_start..end_end,
      };

      Some((position, command))
    }
    None => None,
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple() {
    let (position, command) =
      parse_codeblock("```present echo bar\n```", 0..22)
        .unwrap()
        .unwrap();

    assert_eq!(
      command,
      Command::from(vec!["present".into(), "echo".into(), "bar".into()])
        .unwrap()
        .unwrap()
    );

    assert_eq!(
      position,
      Position {
        start: 0..19,
        end: 19..21
      }
    );
  }

  #[test]
  fn with_exterior_content() {
    let (position, command) =
      parse_codeblock("foo\n\n```present echo bar\n```\n\nbaz", 5..29)
        .unwrap()
        .unwrap();

    assert_eq!(
      command,
      Command::from(vec!["present".into(), "echo".into(), "bar".into()])
        .unwrap()
        .unwrap()
    );

    assert_eq!(
      position,
      Position {
        start: 5..24,
        end: 24..28
      }
    );
  }
}
