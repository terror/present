use clap::Parser;

mod arguments;
mod path_ext;
mod walker;

fn main() {
  if let Err(error) = arguments::Arguments::parse().run() {
    eprintln!("error: {error}");
    std::process::exit(1);
  }
}
