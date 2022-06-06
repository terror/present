use {crate::arguments::Arguments, clap::Parser};

mod arguments;
mod path_ext;
mod walker;

fn main() {
  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
    std::process::exit(1);
  }
}
