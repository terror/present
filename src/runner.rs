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
    let mut files = self.files.clone();

    files
      .iter_mut()
      .map(|file| file.apply_edit())
      .collect::<Result<Vec<_>, _>>()?;

    Ok(())
  }
}
