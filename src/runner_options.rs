use super::*;

#[derive(Debug, Clone, StructOpt)]
pub(crate) struct RunnerOptions {
  #[clap(long, help = "Modify documents in place.")]
  pub(crate) in_place: bool,
  #[clap(long, help = "Interactively present markdown documents.")]
  pub(crate) interactive: bool,
  #[clap(long, help = "Remove commands within markdown documents.")]
  pub(crate) remove: bool,
}
