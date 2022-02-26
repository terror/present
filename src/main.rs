use crate::common::*;

mod arguments;
mod codeblock;
mod command;
mod common;
mod diff;
mod directory;
mod error;
mod file;
mod parser;
mod path_ext;
mod position;
mod prompt;
mod rope_ext;
mod runner;

fn main() {
  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
    process::exit(1);
  }
}
