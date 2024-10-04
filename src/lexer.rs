use crate::{Error, Result};

#[derive(Debug)]
pub(crate) struct Lexer<'a> {
  src: &'a str,
}

impl<'a> Lexer<'a> {
  fn new(src: &'a str) -> Self {
    Self { src }
  }

  pub(crate) fn lex(src: &'a str) -> Result<Vec<String>> {
    Lexer::new(src).tokenize()
  }

  fn tokenize(&self) -> Result<Vec<String>> {
    let mut tokens = Vec::new();
    let mut chars = self.src.chars().peekable();

    let mut current_token = String::new();

    while let Some(ch) = chars.next() {
      match ch {
        '\'' | '"' => {
          if !current_token.is_empty() {
            tokens.push(current_token);
            current_token = String::new();
          }

          let quoted_string = self.parse_quoted_string(ch, &mut chars)?;

          tokens.push(quoted_string);
        }
        ' ' | '\t' => {
          if !current_token.is_empty() {
            tokens.push(current_token);
            current_token = String::new();
          }
        }
        '\\' => {
          if let Some(next_ch) = chars.next() {
            current_token.push(next_ch);
          }
        }
        _ => {
          current_token.push(ch);
        }
      }
    }

    if !current_token.is_empty() {
      tokens.push(current_token);
    }

    Ok(tokens)
  }

  fn parse_quoted_string(
    &self,
    quote: char,
    chars: &mut std::iter::Peekable<std::str::Chars>,
  ) -> Result<String> {
    let mut result = String::new();
    let mut escaped = false;

    while let Some(ch) = chars.next() {
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
        ch if ch == quote => return Ok(result),
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
    Lexer::new(src).tokenize()
  }

  #[test]
  fn tokenize_single() {
    assert_eq!(
      lex("-c 'for i in {1..10}; do echo $i; done'").unwrap(),
      vec!["-c", "for i in {1..10}; do echo $i; done"]
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
  fn tokenize_mixed() {
    assert_eq!(
      lex("a 'b' c 'de' f g \"h i\"").unwrap(),
      vec!["a", "b", "c", "de", "f", "g", "h i"]
    );
  }

  #[test]
  fn ignore_empty() {
    assert_eq!(lex("a     'bc'").unwrap(), vec!["a", "bc"]);
  }

  #[test]
  fn unmatched_delimiter() {
    assert!(lex("-c 'echo foo").is_err());
  }

  #[test]
  fn escaped_quotes() {
    assert_eq!(
      lex(r#"echo "Hello \"World\"""#).unwrap(),
      vec!["echo", r#"Hello "World""#]
    );
  }

  #[test]
  fn nested_quotes() {
    assert_eq!(
      lex(r#"echo "outer 'inner' outer""#).unwrap(),
      vec!["echo", r#"outer 'inner' outer"#]
    );
  }

  #[test]
  fn complex_command() {
    assert_eq!(
      lex(r#"bash -c "echo 'hello world' | tr ' ' '\n' | sort | uniq -c | sort -nr""#).unwrap(),
      vec!["bash", "-c", "echo 'hello world' | tr ' ' '\n' | sort | uniq -c | sort -nr"]
    );
  }

  #[test]
  fn escaped_characters() {
    assert_eq!(
      lex(r#"echo "Hello\nWorld\t\"\\" 'Single\'Quote'"#).unwrap(),
      vec!["echo", "Hello\nWorld\t\"\\", "Single'Quote"]
    );
  }
}
