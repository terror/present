// std
pub(crate) use std::{
  env, fs,
  io::{self, Write},
  ops::Range,
  path::PathBuf,
  process, str,
};

// dependencies
pub(crate) use {
  clap::Parser as StructOpt,
  console::Style,
  pulldown_cmark::{CodeBlockKind, Event, Parser as MarkdownParser, Tag},
  ropey::Rope,
  similar::{ChangeTag, TextDiff},
  snafu::Snafu,
  termimad::print_inline,
  walkdir::WalkDir,
};

// structs and enums
pub(crate) use crate::{
  arguments::Arguments,
  codeblock::Codeblock,
  command::Command,
  diff::Diff,
  directory::{Directory, DirectoryOptions},
  error::Error,
  file::File,
  parser::Parser,
  position::Position,
  runner::{Runner, RunnerOptions},
};

// functions
pub(crate) use crate::prompt::prompt;

// traits
pub(crate) use crate::{path_ext::PathExt, rope_ext::RopeExt};

// type aliases
pub(crate) type Result<T = (), E = Error> = std::result::Result<T, E>;
