use crate::{common::*, Codeblock, Command, Position, Result};

#[derive(Debug, Clone)]
pub(crate) struct Parser<'a> {
  src: &'a str,
}

impl<'a> Parser<'a> {
  pub(crate) fn new(src: &'a str) -> Self {
    Self { src }
  }

  pub(crate) fn parse(&self) -> Result<Vec<Codeblock>> {
    let mut parser = MarkdownParser::new(self.src).into_offset_iter();

    let mut codeblocks = Vec::new();

    while let Some((event, range)) = parser.next() {
      if !matches!(
        event,
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_)))
      ) {
        continue;
      }

      let src = &self.src[range.clone()];

      let fence = char::from(src.as_bytes()[0]);

      let (info, start) = src
        .split_once('\n')
        .map_or((src, range.end), |(info, body)| {
          (info, range.end - body.len())
        });

      let arguments = info
        .trim_start_matches(fence)
        .trim_end_matches('\r')
        .split(' ')
        .map(str::to_owned)
        .collect();

      let Some(command) = Command::from(arguments)? else {
        continue;
      };

      let end = parser
        .by_ref()
        .take_while(|(event, _)| {
          !matches!(event, Event::End(TagEnd::CodeBlock))
        })
        .last()
        .map_or(start, |(_, range)| range.end);

      let end = if self.src[end..range.end].contains(fence) {
        end
      } else {
        range.end
      };

      let block_end = if end < range.end {
        range.end
          + match &self.src.as_bytes()[range.end..] {
            [b'\r', b'\n', ..] => 2,
            [b'\r' | b'\n', ..] => 1,
            _ => 0,
          }
      } else {
        range.end
      };

      codeblocks.push(Codeblock {
        command,
        position: Position {
          block: range.start..block_end,
          body: start..end,
        },
      });
    }

    Ok(codeblocks)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ignore_unrelated_codeblocks() {
    assert_eq!(
      Parser::new(
        "foo\n\n~~~```present echo foo\nbar\n~~~\n\n```bar\nbaz\n```\n\n    present echo foo\n",
      )
      .parse()
      .unwrap(),
      Vec::new(),
    );
  }

  #[test]
  fn parse_codeblocks() {
    #[track_caller]
    fn case(
      src: &str,
      argument: &str,
      block: Range<usize>,
      body: Range<usize>,
    ) {
      assert_eq!(
        Parser::new(src).parse().unwrap(),
        vec![Codeblock {
          command: Command::from(vec![
            "present".into(),
            "echo".into(),
            argument.into(),
          ])
          .unwrap()
          .unwrap(),
          position: Position { block, body },
        }],
      );
    }

    case("```present echo foo\n```", "foo", 0..23, 20..20);
    case("```present echo foo", "foo", 0..19, 19..19);
    case("```present echo foo\n", "foo", 0..20, 20..20);
    case("```present echo foo\nbar", "foo", 0..23, 20..23);
    case("  ```present echo foo\nbar\n  ", "foo", 2..28, 22..28);
    case("  ```present echo foo\n  ", "foo", 2..24, 22..24);
    case("> ```present echo foo\n> bar\n> ", "foo", 2..30, 22..30);
    case("> ```present echo foo\n> bar\n\nbaz", "foo", 2..28, 22..28);
    case("🚀\n\n```present echo 🚀\nbar\n```\n", "🚀", 6..35, 27..31);
    case("~~~present echo foo\nbar\n~~~~~  \n", "foo", 0..32, 20..24);

    case(
      "foo\n\n```present echo bar\n```\n\nbaz",
      "bar",
      5..29,
      25..25,
    );

    case(
      "foo\r\n\r\n```present echo bar\r\nbaz\r\n```\r\n\r\nqux",
      "bar",
      7..38,
      28..33,
    );

    case(
      "  ```present echo foo\n  bar\n  ```\n",
      "foo",
      2..34,
      22..28,
    );
  }
}
