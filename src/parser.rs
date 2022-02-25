use super::*;

#[derive(Debug, Clone)]
pub(crate) struct Parser<'a> {
  src: &'a str,
}

impl<'a> Parser<'a> {
  pub(crate) fn new(src: &'a str) -> Self {
    Self { src }
  }

  pub(crate) fn commands(&self) -> Result<Vec<Command>> {
    let parser = MarkdownParser::new(self.src);

    let events = parser.into_offset_iter().filter(|event| {
      matches!(
        event,
        (Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_))), _)
      )
    });

    Ok(
      events
        .filter_map(|event| Command::from(Chunk::new(self.src, event.1)))
        .collect::<Vec<Command>>(),
    )
  }
}
