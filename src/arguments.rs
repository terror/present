use {
  crate::walker::{Walker, WalkerOptions},
  present::{File, Result},
};

use clap::Parser as StructOpt;

#[derive(Debug, StructOpt)]
#[clap(about = env!("CARGO_PKG_DESCRIPTION"), version = env!("CARGO_PKG_VERSION"))]
pub(crate) struct Arguments {
  #[clap(flatten)]
  walker_options: WalkerOptions,
  #[clap(long, help = "Modify documents in place.")]
  pub(crate) in_place: bool,
  #[clap(long, help = "Interactively present markdown documents.")]
  pub(crate) interactive: bool,
  #[clap(long, help = "Pretty print documents to the terminal.")]
  pub(crate) pretty: bool,
  #[clap(long, help = "Remove commands within markdown documents.")]
  pub(crate) remove: bool,
}

impl Arguments {
  pub(crate) fn run(self) -> Result {
    Walker::new(self.walker_options)?
      .files()
      .try_for_each(|file| {
        let mut file = File::new(file)?
          .remove(self.remove)
          .interactive(self.interactive);

        file.present()?;

        match self.in_place {
          true => file.save()?,
          false => file.print(self.pretty),
        };

        Ok(())
      })
  }
}
