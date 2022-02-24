use super::*;

#[derive(Debug)]
pub(crate) struct Directory {
  options: DirectoryOptions,
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

    // If the provided path is a file, our
    // 'directory' contains this single file.
    if path.is_file() {
      return Ok(vec![File::new(path.clone())?]);
    }

    let mut walker = WalkDir::new(&path);

    // If we aren't recursing on a directory,
    // set the walkers max depth to one.
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
