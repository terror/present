use crate::path_ext::PathExt;
use clap::Parser as StructOpt;
use present::Result;
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct Walker {
  path: PathBuf,
  recursive: bool,
}

#[derive(Debug, Clone, StructOpt)]
pub(crate) struct WalkerOptions {
  #[clap(long, help = "A file or directory path to present.")]
  pub(crate) path: Option<PathBuf>,
  #[clap(long, help = "Recursively present markdown documents.")]
  pub(crate) recursive: bool,
}

impl Walker {
  pub(crate) fn new(options: WalkerOptions) -> Result<Self> {
    let path = options
      .path
      .unwrap_or(std::env::current_dir()?)
      .validate()?;

    Ok(Self {
      path,
      recursive: options.recursive,
    })
  }

  pub(crate) fn files(&self) -> impl Iterator<Item = PathBuf> {
    let mut walker = walkdir::WalkDir::new(&self.path);

    if !self.recursive {
      walker = walker.max_depth(1);
    }

    walker
      .into_iter()
      .filter_map(|entry| entry.ok())
      .map(|entry| entry.into_path())
      .filter(|entry| entry.is_markdown())
  }
}
