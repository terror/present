use crate::{codeblock::parse_codeblock, common::*, Command, Position};

#[derive(Debug, Clone)]
pub(crate) struct Parser<'a> {
  src: &'a str,
}

impl<'a> Parser<'a> {
  pub(crate) fn new(src: &'a str) -> Self {
    Self { src }
  }

  pub(crate) fn parse(&self) -> Vec<(Position, Command)> {
    MarkdownParser::new(self.src)
      .into_offset_iter()
      .filter(|event| {
        matches!(
          event,
          (Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_))), _)
        )
      })
      .filter_map(|event| parse_codeblock(self.src, event.1))
      .collect()
  }
}
