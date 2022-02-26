use crate::common::*;

#[derive(Debug, StructOpt)]
#[clap(about = env!("CARGO_PKG_DESCRIPTION"), version = env!("CARGO_PKG_VERSION"))]
pub(crate) struct Arguments {
  #[clap(flatten)]
  directory_options: DirectoryOptions,
  #[clap(flatten)]
  runner_options: RunnerOptions,
}

impl Arguments {
  pub(crate) fn run(self) -> Result {
    Runner::new(
      Directory::new(self.directory_options).files()?,
      self.runner_options,
    )
    .run()
  }
}
