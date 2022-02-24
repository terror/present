use super::*;

const MARKDOWN: &str = "md";

pub(crate) trait PathExt {
  fn is_markdown(&self) -> bool;
  fn validate(&self) -> Result<PathBuf>;
}

impl PathExt for PathBuf {
  fn is_markdown(&self) -> bool {
    self.extension().unwrap_or_default() == MARKDOWN
  }

  fn validate(&self) -> Result<PathBuf> {
    let path = self.clone();
    match self.exists() {
      true => Ok(path),
      _ => Err(Error::PathDoesNotExist { path }),
    }
  }
}
