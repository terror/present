use super::*;

#[derive(Debug)]
pub(crate) struct Lexer<'a> {
  src: &'a str,
}

impl<'a> Lexer<'a> {
  fn new(src: &'a str) -> Self {
    Self { src }
  }

  pub(crate) fn lex(src: &'a str) -> Result<Vec<String>> {
    Lexer::new(&src.replace("\r\n", "\n")).tokenize()
  }

  fn tokenize(&self) -> Result<Vec<String>> {
    let mut tokens = Vec::new();
    let mut chars = self.src.chars().peekable();

    let mut current_token = None;

    while let Some(ch) = chars.next() {
      match ch {
        '\'' | '"' => {
          current_token
            .get_or_insert_with(String::new)
            .push_str(&self.parse_quoted_string(ch, &mut chars)?);
        }
        ' ' | '\t' | '\r' => {
          if let Some(token) = current_token.take() {
            tokens.push(token);
          }
        }
        '\\' => {
          if let Some(next_ch) = chars.next() {
            current_token.get_or_insert_with(String::new).push(next_ch);
          }
        }
        _ => {
          current_token.get_or_insert_with(String::new).push(ch);
        }
      }
    }

    if let Some(token) = current_token {
      tokens.push(token);
    }

    let normalized_tokens = tokens
      .into_iter()
      .map(|token| token.replace("\r\n", "\n"))
      .collect();

    Ok(normalized_tokens)
  }

  fn parse_quoted_string(
    &self,
    quote: char,
    chars: &mut Peekable<Chars>,
  ) -> Result<String> {
    let mut result = String::new();
    let mut escaped = false;

    for ch in chars.by_ref() {
      match ch {
        _ if escaped => {
          match ch {
            '\\' | '\'' | '"' => result.push(ch),
            'n' => result.push('\n'),
            't' => result.push('\t'),
            'r' => result.push('\r'),
            _ => {
              result.push('\\');
              result.push(ch);
            }
          }
          escaped = false;
        }
        '\\' => escaped = true,
        ch if ch == quote => return Ok(result.replace("\r\n", "\n")),
        _ => result.push(ch),
      }
    }

    Err(Error::LexError {
      message: "Unmatched delimiter".into(),
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn lex(src: &str) -> Result<Vec<String>> {
    Lexer::lex(src)
  }

  #[test]
  fn adjacent_fragments() {
    #[track_caller]
    fn case(src: &str, expected: &[&str]) {
      assert_eq!(lex(src).unwrap(), expected);
    }

    case(r#"foo"bar"'baz'"#, &["foobarbaz"]);

    case(
      r#""foo"bar 'foo'"bar" baz"qux""#,
      &["foobar", "foobar", "bazqux"],
    );

    case(r#"foo" bar "baz"#, &["foo bar baz"]);
  }

  #[test]
  fn complex_command() {
    assert_eq!(
      lex(r#"bash -c "echo 'hello world' | tr ' ' '\n' | sort | uniq -c | sort -nr""#).unwrap(),
      vec!["bash", "-c", "echo 'hello world' | tr ' ' '\n' | sort | uniq -c | sort -nr"]
    );
  }

  #[test]
  fn empty_arguments() {
    #[track_caller]
    fn case(src: &str, expected: &[&str]) {
      assert_eq!(lex(src).unwrap(), expected);
    }

    case(r#"'' """#, &["", ""]);
    case(r#"''"" ""''"#, &["", ""]);
    case(r#"foo'' ''foo foo""bar"#, &["foo", "foo", "foobar"]);
    case(" \t\r ", &[]);
  }

  #[test]
  fn escaped_characters() {
    #[track_caller]
    fn case(src: &str, expected: &[&str]) {
      assert_eq!(lex(src).unwrap(), expected);
    }

    case(
      r#"echo "Hello\nWorld\t\"\\" 'Single\'Quote'"#,
      &["echo", "Hello\nWorld\t\"\\", "Single'Quote"],
    );

    case(
      r#"'foo\n\t\r\q\\\'\"bar' "foo\n\t\r\q\\\'\"bar""#,
      &["foo\n\t\r\\q\\'\"bar", "foo\n\t\r\\q\\'\"bar"],
    );

    case(
      r#"foo\ bar \n\t\r\q\\\'\" foo\"#,
      &["foo bar", "ntrq\\'\"", "foo"],
    );

    case("\\", &[]);
    case(r#"''\"#, &[""]);
  }

  #[test]
  fn escaped_quotes() {
    assert_eq!(
      lex(r#"echo "Hello \"World\"""#).unwrap(),
      vec!["echo", r#"Hello "World""#]
    );
  }

  #[test]
  fn ignore_empty() {
    assert_eq!(lex("a     'bc'").unwrap(), vec!["a", "bc"]);
  }

  #[test]
  fn nested_quotes() {
    assert_eq!(
      lex(r#"echo "outer 'inner' outer""#).unwrap(),
      vec!["echo", r#"outer 'inner' outer"#]
    );
  }

  #[test]
  fn tokenize_mixed() {
    assert_eq!(
      lex("a 'b' c 'de' f g \"h i\"").unwrap(),
      vec!["a", "b", "c", "de", "f", "g", "h i"]
    );
  }

  #[test]
  fn tokenize_multiple() {
    assert_eq!(
      lex("-c 'echo foo' 'echo bar'").unwrap(),
      vec!["-c", "echo foo", "echo bar"]
    );
  }

  #[test]
  fn tokenize_single() {
    assert_eq!(
      lex("-c 'for i in {1..10}; do echo $i; done'").unwrap(),
      vec!["-c", "for i in {1..10}; do echo $i; done"]
    );
  }

  #[test]
  fn unmatched_delimiter() {
    assert!(lex("-c 'echo foo").is_err());
  }

  #[test]
  fn windows_line_endings() {
    #[track_caller]
    fn case(src: &str) {
      assert_eq!(lex(src).unwrap(), ["foo\nbar"]);
    }

    case("\"foo\r\nbar\"");
    case("foo\r\nbar");
    case(r#"'foo\r\nbar'"#);
    case(r#"'foo\r''\nbar'"#);
  }
}
