use super::*;

#[derive(Debug, Clone, StructOpt)]
pub(crate) struct DirectoryOptions {
  #[clap(long, help = "A file or directory path to present.")]
  pub(crate) path: Option<PathBuf>,
  #[clap(long, help = "Recursively present markdown documents.")]
  pub(crate) recursive: bool,
}
