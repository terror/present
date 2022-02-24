use super::*;

#[derive(Debug, Clone)]
pub(crate) struct Parser<'a> {
  src: &'a str,
}

#[derive(Debug, Clone)]
pub(crate) struct Chunk {
  pub(crate) src: String,
  pub(crate) start: Position,
  pub(crate) end: Position,
}

impl Chunk {
  fn new(src: String, start: Position, end: Position) -> Self {
    Self { src, start, end }
  }
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
        (Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(_))), range) => {
          commands.push(Position::from(range));
        }
        _ => {}
      }
    }

    Ok(
      commands
        .chunks_exact(2)
        .map(|chunk| {
          Chunk::new(self.src.to_owned(), chunk[0].clone(), chunk[1].clone())
        })
        .collect::<Vec<Chunk>>()
        .iter()
        .map(Command::from)
        .filter(|command| command.is_some())
        .flatten()
        .collect::<Vec<Command>>(),
    )
  }
}
