use super::*;

pub(crate) struct Parser<'a> {
  src: &'a str,
}

impl<'a> Parser<'a> {
  pub(crate) fn new(src: &'a str) -> Self {
    Self { src }
  }

  pub(crate) fn extract_commands(&self) -> Result<Vec<Command>> {
    let parser = MarkdownParser::new(self.src);

    let mut start = 0;
    let mut current_command = None;
    let mut commands = Vec::new();

    for event in parser.into_offset_iter() {
      match event {
        (Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_))), range) => {
          if let Some(command) =
            Command::parse(&self.src[range.start..range.end])
          {
            start = range.start;
            current_command = Some(command);
          }
        }
        (Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(_))), range) => {
          if let Some(command) = &current_command {
            commands.push(
              command
                .clone()
                .set_position(Position::new(start, range.end)),
            );
          }
        }
        _ => {}
      }
    }

    Ok(commands)
  }
}
