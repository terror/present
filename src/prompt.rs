use super::*;

pub(crate) fn prompt(message: &str) -> Result<String> {
  eprint!("{} › ", Style::new().apply_to(message).bold());
  let mut input = String::new();
  io::stdout().flush()?;
  io::stdin().read_line(&mut input)?;
  Ok(input.as_str().to_lowercase().trim().to_owned())
}
