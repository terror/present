use {
  crate::{
    arguments::Arguments,
    command::Command,
    directory::Directory,
    directory_options::DirectoryOptions,
    error::Error,
    file::{Diff, File},
    parser::{Chunk, Parser},
    path_ext::PathExt,
    position::Position,
    runner::Runner,
    runner_options::RunnerOptions,
  },
  clap::Parser as StructOpt,
  pulldown_cmark::{CodeBlockKind, Event, Parser as MarkdownParser, Tag},
  ropey::Rope,
  snafu::Snafu,
  std::{
    env,
    fmt::{self, Display, Formatter},
    fs, io,
    ops::Range,
    path::PathBuf,
    process,
  },
  walkdir::WalkDir,
};

mod arguments;
mod command;
mod directory;
mod directory_options;
mod error;
mod file;
mod parser;
mod path_ext;
mod position;
mod runner;
mod runner_options;

type Result<T = (), E = Error> = std::result::Result<T, E>;

fn main() {
  if let Err(error) = Arguments::parse().run() {
    println!("error: {error}");
    process::exit(1);
  }
}
