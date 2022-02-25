use {
  crate::{
    arguments::Arguments,
    chunk::Chunk,
    codeblock::Codeblock,
    command::Command,
    diff::Diff,
    directory::{Directory, DirectoryOptions},
    error::Error,
    file::File,
    parser::Parser,
    path_ext::PathExt,
    rope_ext::RopeExt,
    runner::{Runner, RunnerOptions},
  },
  clap::Parser as StructOpt,
  pulldown_cmark::{CodeBlockKind, Event, Parser as MarkdownParser, Tag},
  ropey::Rope,
  snafu::Snafu,
  std::{env, fs, io, ops::Range, path::PathBuf, process, str},
  termimad::print_inline,
  walkdir::WalkDir,
};

mod arguments;
mod chunk;
mod codeblock;
mod command;
mod diff;
mod directory;
mod error;
mod file;
mod parser;
mod path_ext;
mod rope_ext;
mod runner;

type Result<T = (), E = Error> = std::result::Result<T, E>;

fn main() {
  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
    process::exit(1);
  }
}
