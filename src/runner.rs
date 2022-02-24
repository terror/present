use super::*;

#[derive(Debug)]
pub(crate) struct Runner {
  files: Vec<File>,
  options: RunnerOptions,
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
