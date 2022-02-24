use super::*;

#[derive(Debug)]
pub(crate) struct Directory {
  options: DirectoryOptions,
}

#[derive(Debug, Clone, StructOpt)]
pub(crate) struct DirectoryOptions {
  #[clap(long, help = "A file or directory path to present.")]
  pub(crate) path: Option<PathBuf>,
  #[clap(long, help = "Recursively present markdown documents.")]
  pub(crate) recursive: bool,
}

impl Directory {
  pub(crate) fn new(options: DirectoryOptions) -> Self {
    Self { options }
  }

  pub(crate) fn files(&self) -> Result<Vec<File>> {
    let path = self
      .options
      .path
      .clone()
      .unwrap_or(env::current_dir()?)
      .validate()?;

    if path.is_file() {
      return Ok(vec![File::new(path)?]);
    }

    let mut walker = WalkDir::new(&path);

    if !self.options.recursive {
      walker = walker.max_depth(1);
    }

    walker
      .into_iter()
      .collect::<Result<Vec<_>, _>>()?
      .iter()
      .cloned()
      .map(|entry| entry.into_path())
      .filter(|entry| entry.is_markdown())
      .map(File::new)
      .collect::<Result<Vec<_>, _>>()
  }
}
