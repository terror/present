use crate::{codeblock::parse_codeblock, common::*, Command, Position, Result};

#[derive(Debug, Clone)]
pub(crate) struct Parser<'a> {
  src: &'a str,
}

impl<'a> Parser<'a> {
  pub(crate) fn new(src: &'a str) -> Self {
    Self { src }
  }

  pub(crate) fn parse(&self) -> Result<Vec<(Position, Command)>> {
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

    let mut parsed_codeblocks = Vec::new();

    for range in ranges {
      if let Some(parsed_codeblock) = parse_codeblock(self.src, range)? {
        parsed_codeblocks.push(parsed_codeblock);
      }
    }

    Ok(parsed_codeblocks)
  }
}
