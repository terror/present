use crate::common::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(display(
    "Command at position: {:?} failed to run with message: {}",
    range,
    message
  ))]
  Command {
    range: Range<usize>,
    message: String,
  },

  #[snafu(context(false), display("IO Error: {}", source))]
  Io { source: io::Error },

  #[snafu(display("{}", path.display()))]
  PathDoesNotExist { path: PathBuf },

  #[snafu(context(false), display("Utf8 Error: {}", source))]
  Utf8 { source: str::Utf8Error },

  #[snafu(context(false), display("Walkdir Error: {}", source))]
  Walkdir { source: walkdir::Error },
}
