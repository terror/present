use crate::{common::*, rope_ext::*, Codeblock, Command, Position, Result};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub(crate) struct Parser<'a> {
  src: &'a str,
}

impl<'a> Parser<'a> {
  pub(crate) fn new(src: &'a str) -> Self {
    Self { src }
  }

  pub(crate) fn parse(&self) -> Result<Vec<Codeblock>> {
    let ranges = MarkdownParser::new(self.src)
      .into_offset_iter()
      .filter(|event| {
        matches!(
          event,
          (Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_))), _)
        )
      })
      .map(|event| event.1)
      .collect::<Vec<Range<usize>>>();

    let codeblocks = ranges
      .iter()
      .map(|range| self.parse_codeblock(range.clone()))
      .collect::<Result<Vec<_>, _>>()?;

    Ok(
      codeblocks
        .iter()
        .filter_map(|codeblock| codeblock.clone())
        .collect(),
    )
  }

  fn parse_codeblock(&self, range: Range<usize>) -> Result<Option<Codeblock>> {
    let start_start = range.start;
    let mut start_end = start_start;

    let src_graphemes: Vec<&str> = self.src.graphemes(true).collect();

    while let Some(grapheme) =
      src_graphemes.get(byte_index_to_grapheme_index(self.src, start_end))
    {
      match *grapheme {
        "`" => {
          start_end = grapheme_index_to_byte_index(
            self.src,
            byte_index_to_grapheme_index(self.src, start_end) + 1,
          )
        }
        _ => break,
      }
    }

    while let Some(grapheme) =
      src_graphemes.get(byte_index_to_grapheme_index(self.src, start_end))
    {
      match *grapheme {
        "`" | "\n" => break,
        _ => {
          start_end = grapheme_index_to_byte_index(
            self.src,
            byte_index_to_grapheme_index(self.src, start_end) + 1,
          )
        }
      }
    }

    let end_end = range.end - 1;
    let mut end_start = end_end;

    while let Some(grapheme) =
      src_graphemes.get(byte_index_to_grapheme_index(self.src, end_start))
    {
      match *grapheme {
        "`" => break,
        _ => {
          end_start = grapheme_index_to_byte_index(
            self.src,
            byte_index_to_grapheme_index(self.src, end_start) - 1,
          )
        }
      }
    }

    while let Some(grapheme) =
      src_graphemes.get(byte_index_to_grapheme_index(self.src, end_start))
    {
      match *grapheme {
        "`" => {
          end_start = grapheme_index_to_byte_index(
            self.src,
            byte_index_to_grapheme_index(self.src, end_start) - 1,
          )
        }
        _ => break,
      }
    }

    let arguments = self.src[start_start..start_end]
      .trim_start_matches('`')
      .split(' ')
      .map(|s| s.into())
      .collect::<Vec<String>>();

    Ok(match Command::from(arguments)? {
      Some(command) => {
        let position = Position {
          start: start_start..start_end,
          end: end_start..end_end,
        };

        Some(Codeblock { command, position })
      }
      None => None,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_codeblock_simple() {
    let parser = Parser::new("```present echo bar\n```");

    let codeblock = parser.parse_codeblock(0..22).unwrap().unwrap();

    assert_eq!(
      codeblock.command,
      Command::from(vec!["present".into(), "echo".into(), "bar".into()])
        .unwrap()
        .unwrap()
    );

    assert_eq!(
      codeblock.position,
      Position {
        start: 0..19,
        end: 19..21
      }
    );
  }

  #[test]
  fn parse_codeblock_with_exterior_content() {
    let parser = Parser::new("foo\n\n```present echo bar\n```\n\nbaz");

    let codeblock = parser.parse_codeblock(5..29).unwrap().unwrap();

    assert_eq!(
      codeblock.command,
      Command::from(vec!["present".into(), "echo".into(), "bar".into()])
        .unwrap()
        .unwrap()
    );

    assert_eq!(
      codeblock.position,
      Position {
        start: 5..24,
        end: 24..28
      }
    );
  }

  #[test]
  fn parse_codeblock_with_unicode() {
    let parser = Parser::new("```present echo 🚀\n```");

    let codeblock = parser.parse_codeblock(0..23).unwrap().unwrap();

    assert_eq!(
      codeblock.command,
      Command::from(vec!["present".into(), "echo".into(), "🚀".into()])
        .unwrap()
        .unwrap()
    );

    assert_eq!(
      codeblock.position,
      Position {
        start: 0..20,
        end: 20..22
      }
    );
  }
}
