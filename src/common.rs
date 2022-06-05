//! Contains commonly used stuff from external crates

// std
pub(crate) use std::{
  fs,
  io::{self, Write},
  ops::Range,
  path::PathBuf,
  process, str,
};

// dependencies
pub(crate) use {
  console::Style,
  pulldown_cmark::{CodeBlockKind, Event, Parser as MarkdownParser, Tag},
  ropey::Rope,
  similar::{ChangeTag, TextDiff},
  snafu::Snafu,
  termimad::print_inline,
};
