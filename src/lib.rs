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
pub use crate::{diff::Diff, error::Error, file::File};

// Public only to crate
pub(crate) use crate::{
  command::Command, parser::Parser, prompt::prompt, rope_ext::RopeExt, position::Position,
};

/// Present's internal result type
pub type Result<T = (), E = Error> = std::result::Result<T, E>;
