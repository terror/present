use crate::common::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(display(
    "Program {} failed to execute with message: {}",
    program,
    message
  ))]
  Command { program: String, message: String },

  #[snafu(context(false), display("IO Error: {}", source))]
  Io { source: io::Error },

  #[snafu(display("Path does not exist: {}", path.display()))]
  PathDoesNotExist { path: PathBuf },

  #[snafu(context(false), display("Utf8 Error: {}", source))]
  Utf8 { source: std::string::FromUtf8Error },

  #[snafu(context(false), display("Walkdir Error: {}", source))]
  Walkdir { source: walkdir::Error },
}
