use {
  present::{Error, Result},
  std::path::PathBuf,
};

const MARKDOWN: &str = "md";

pub(crate) trait PathExt {
  fn is_markdown(&self) -> bool;
  fn validate(self) -> Result<Self>
  where
    Self: Sized;
}

impl PathExt for PathBuf {
  fn is_markdown(&self) -> bool {
    self.extension().unwrap_or_default() == MARKDOWN
  }

  fn validate(self) -> Result<Self> {
    match self.exists() {
      true => Ok(self),
      _ => Err(Error::PathDoesNotExist { path: self }),
    }
  }
}
