use super::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(context(false), display("IO Error: {}", source))]
  Io { source: io::Error },
  #[snafu(display("Path does not exist: {}", path.display()))]
  PathDoesNotExist { path: PathBuf },
  #[snafu(display("Malformed command starting at {} and ending at {}", position.start, position.end))]
  MalformedCommand { position: Position },
  #[snafu(context(false), display("Walkdir Error: {}", source))]
  Walkdir { source: walkdir::Error },
}
