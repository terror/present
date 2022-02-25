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

    let mut commands = Vec::new();

    for event in parser.into_offset_iter() {
      match event {
        (Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_))), range) => {
          commands.push(Position::from(range));
        }
        _ => {}
      }
    }

    Ok(
      commands
        .iter()
        .map(|position| Chunk::new(self.src, position.clone()))
        .collect::<Vec<Chunk>>()
        .iter()
        .map(Command::from)
        .filter(|command| command.is_some())
        .flatten()
        .collect::<Vec<Command>>(),
    )
  }
}
