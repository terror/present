//! present is a tool that lets you interpolate the standard output of arbitrary
//! scripts that get interpreted by the shell into your markdown documents.

mod codeblock;
mod command;
mod common;
mod diff;
mod error;
mod file;
mod parser;
mod position;
mod prompt;
mod rope_ext;

// Publicly exposed
pub use crate::{
  command::Command, diff::Diff, error::Error, file::File, parser::Parser,
  position::Position,
};

// Public only to crate
pub(crate) use crate::{prompt::prompt, rope_ext::RopeExt};

// type aliases
pub type Result<T = (), E = Error> = std::result::Result<T, E>;
