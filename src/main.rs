use {
  arguments::Arguments,
  clap::Parser,
  path_ext::PathExt,
  present::{Error, File, Result},
  std::{env, path::PathBuf, process},
  walkdir::WalkDir,
  walker::{Walker, WalkerOptions},
};

mod arguments;
mod path_ext;
mod walker;

fn main() {
  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
    process::exit(1);
  }
}
