//! Contains commonly used stuff from external crates

pub(crate) use std::{
  fs,
  io::{self, Write},
  ops::Range,
  path::PathBuf,
  process, str,
};

pub(crate) use {
  console::Style,
  pulldown_cmark::{CodeBlockKind, Event, Parser as MarkdownParser, Tag},
  ropey::Rope,
  similar::{ChangeTag, TextDiff},
  snafu::Snafu,
  termimad::print_inline,
  unicode_segmentation::UnicodeSegmentation,
};
