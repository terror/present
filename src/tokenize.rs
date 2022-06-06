use crate::{Error, Result};

pub(crate) trait Tokenize {
  fn tokenize(self) -> Result<Vec<String>>;
}

impl Tokenize for String {
  fn tokenize(self) -> Result<Vec<String>> {
    let mut tokens = Vec::new();

    let mut chars = self.chars().peekable();

    while let Some(ch) = chars.next() {
      match ch {
        '\'' | '"' => {
          let mut group = String::new();

          chars
            .clone()
            .collect::<Vec<char>>()
            .iter()
            .find(|next| **next == ch)
            .ok_or(Error::TokenizeError {
              message: "Unmatched delimeter".into(),
            })?;

          while let Some(next) = chars.next() {
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

  #[test]
  fn tokenize_single() {
    assert_eq!(
      "-c 'for i in {1..10}; do echo $i; done'"
        .to_string()
        .tokenize()
        .unwrap(),
      vec!["-c", "for i in {1..10}; do echo $i; done"]
    );
  }

  #[test]
  fn tokenize_multiple() {
    assert_eq!(
      "-c 'echo foo' 'echo bar'".to_string().tokenize().unwrap(),
      vec!["-c", "echo foo", "echo bar"]
    );
  }

  #[test]
  fn tokenize_mixed() {
    assert_eq!(
      "a 'b' c 'de' f g \"h i\"".to_string().tokenize().unwrap(),
      vec!["a", "b", "c", "de", "f", "g", "h i"]
    );
  }

  #[test]
  fn ignore_empty() {
    assert_eq!(
      "a     'bc'".to_string().tokenize().unwrap(),
      vec!["a", "bc"]
    );
  }

  #[test]
  fn unmatched_delimiter() {
    assert!("-c 'echo foo".to_string().tokenize().is_err(),);
  }
}
