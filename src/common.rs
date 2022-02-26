// std
pub(crate) use std::{env, fs, io, ops::Range, path::PathBuf, process, str};

// dependencies
pub(crate) use {
  clap::Parser as StructOpt,
  pulldown_cmark::{CodeBlockKind, Event, Parser as MarkdownParser, Tag},
  ropey::Rope,
  snafu::Snafu,
  termimad::print_inline,
  walkdir::WalkDir,
};

// structs and enums
pub(crate) use crate::{
  arguments::Arguments,
  chunk::Chunk,
  codeblock::Codeblock,
  command::Command,
  diff::Diff,
  directory::{Directory, DirectoryOptions},
  error::Error,
  file::File,
  parser::Parser,
  runner::{Runner, RunnerOptions},
};

// traits
pub(crate) use crate::{path_ext::PathExt, rope_ext::RopeExt};

// type aliases
pub(crate) type Result<T = (), E = Error> = std::result::Result<T, E>;
