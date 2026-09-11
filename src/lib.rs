//! `present` is a simple tool that lets you interpolate the standard output of
//! arbitrary scripts that get interpreted by the shell into your markdown
//! documents.
//!
//! With `present`, you can create a `File` struct by pointing it to a path.
//! This will parse all codeblocks with the `present` prefix, and add them as
//! commands to the struct. From there, you can present the file by using the
//! `File::present` function, which will modify the internal content. From
//! there, you can use the `File::print` or `File::save` functions to print the
//! presented document to stdout or save it back to the original file.
//!
//! ```rust
//! use std::path::PathBuf;
//!
//! let mut file = present::File::new(PathBuf::from("README.md")).unwrap();
//! file.present().unwrap();
//! file.save();
//! ```

#[cfg(target_os = "windows")]
use std::env;

pub use {diff::Diff, error::Error, file::File};

use {
  codeblock::Codeblock,
  command::Command,
  console::Style,
  lexer::Lexer,
  parser::Parser,
  position::Position,
  prompt::prompt,
  pulldown_cmark::{CodeBlockKind, Event, Tag, TagEnd},
  rope_ext::RopeExt,
  ropey::Rope,
  similar::{ChangeTag, TextDiff},
  snafu::Snafu,
  std::{
    fs,
    io::{self, Write},
    iter::Peekable,
    ops::Range,
    path::PathBuf,
    process,
    str::Chars,
    string::FromUtf8Error,
  },
  termimad::print_inline,
};

mod codeblock;
mod command;
mod diff;
mod error;
mod file;
mod lexer;
mod parser;
mod position;
mod prompt;
mod rope_ext;

/// Present's internal result type
pub type Result<T = (), E = Error> = std::result::Result<T, E>;

// Test README
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
