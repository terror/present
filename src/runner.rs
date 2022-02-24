use super::*;

#[derive(Debug)]
pub(crate) struct Runner {
  files: Vec<File>,
  options: RunnerOptions,
}

#[derive(Debug, Clone, StructOpt)]
pub(crate) struct RunnerOptions {
  #[clap(long, help = "Modify documents in place.")]
  pub(crate) in_place: bool,
  #[clap(long, help = "Interactively present markdown documents.")]
  pub(crate) interactive: bool,
  #[clap(long, help = "Remove commands within markdown documents.")]
  pub(crate) remove: bool,
}

impl Runner {
  pub(crate) fn new(files: Vec<File>, options: RunnerOptions) -> Self {
    Self { files, options }
  }

  pub(crate) fn run(&mut self) -> Result {
    self
      .files
      .clone()
      .iter_mut()
      .try_for_each(|file| file.present(self.options.clone()))
  }
}
