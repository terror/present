use super::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(context(false), display("IO Error: {}", source))]
  Io { source: io::Error },

  #[snafu(display("Malformed command starting at {} and ending at {}", position.start, position.end))]
  MalformedCommand { position: Position },

  #[snafu(display("Path does not exist: {}", path.display()))]
  PathDoesNotExist { path: PathBuf },

  #[snafu(context(false), display("Utf8 conversion error: {}", source))]
  Utf8 { source: str::Utf8Error },

  #[snafu(context(false), display("Walkdir Error: {}", source))]
  Walkdir { source: walkdir::Error },
}
