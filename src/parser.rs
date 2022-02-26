use crate::common::*;

#[derive(Debug, Clone)]
pub(crate) struct Parser<'a> {
  src: &'a str,
}

impl<'a> Parser<'a> {
  pub(crate) fn new(src: &'a str) -> Self {
    Self { src }
  }

  pub(crate) fn parse(&self) -> Result<Vec<Command>> {
    Ok(
      MarkdownParser::new(self.src)
        .into_offset_iter()
        .filter(|event| {
          matches!(
            event,
            (Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_))), _)
          )
        })
        .filter_map(|event| Command::from(Codeblock::new(self.src, event.1)))
        .collect::<Vec<Command>>(),
    )
  }
}
