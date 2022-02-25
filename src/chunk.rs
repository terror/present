use super::*;

#[derive(Debug, Clone)]
pub(crate) struct Chunk<'a> {
  pub(crate) src: &'a str,
  pub(crate) start: Position,
  pub(crate) end: Position,
}

impl<'a> Chunk<'a> {
  pub(crate) fn new(src: &'a str, position: Position) -> Self {
    let start_start = position.start;
    let mut start_end = start_start;

    while let Some(ch) = src.chars().nth(start_end) {
      match ch {
        '\n' => {
          start_end += 1;
          break;
        }
        _ => start_end += 1,
      }
    }

    let end_end = position.end;
    let mut end_start = end_end;

    while let Some(ch) = src.chars().nth(end_start - 1) {
      match ch {
        '`' => {
          end_start -= 1;
          continue;
        }
        _ => {
          break;
        }
      }
    }

    Self {
      src,
      start: Position::new(start_start, start_end),
      end: Position::new(end_start, end_end),
    }
  }
}
