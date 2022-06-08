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

    while let Some(ch) = chars.next() {
      match ch {
        '\'' | '"' => {
          let mut group = String::new();

          chars
            .clone()
            .collect::<Vec<char>>()
            .iter()
            .find(|next| **next == ch)
            .ok_or(Error::LexError {
              message: "Unmatched delimeter".into(),
            })?;

          for next in chars.by_ref() {
            match next {
              next if next == ch => break,
              _ => group.push(next),
            }
          }

          chars.next();

          tokens.push(group);
        }
        _ => {
          let mut group = String::new();

          group.push(ch);

          while let Some(next) = chars.peek() {
            match next {
              '\'' | '"' => break,
              _ => {
                group.push(*next);
                chars.next();
              }
            }
          }

          tokens.extend(
            group
              .trim()
              .split(' ')
              .map(|argument| argument.to_owned())
              .collect::<Vec<String>>(),
          );
        }
      }
    }

    Ok(tokens)
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
}
