use crate::common::*;
use crate::{codeblock::parse_codeblock, Command, Position, Result};

#[derive(Debug, Clone)]
pub struct Parser<'a> {
  src: &'a str,
}

impl<'a> Parser<'a> {
  pub(crate) fn new(src: &'a str) -> Self {
    Self { src }
  }

  pub(crate) fn parse(&self) -> Result<Vec<(Position, Command)>> {
    Ok(
      MarkdownParser::new(self.src)
        .into_offset_iter()
        .filter(|event| {
          matches!(
            event,
            (Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_))), _)
          )
        })
        .filter_map(|event| parse_codeblock(self.src, event.1))
        .collect(),
    )
  }
}
