use {
  regex::{Captures, Regex},
  std::{fs, process::Command, str},
};

const REPO: &str = "terror/present";

fn author(pr: u64) -> String {
  eprintln!("#{pr}");

  let output = Command::new("gh")
    .args([
      "pr",
      "view",
      &pr.to_string(),
      "--repo",
      REPO,
      "--json",
      "author",
      "--jq",
      ".author.login",
    ])
    .output()
    .unwrap();

  assert!(
    output.status.success(),
    "{}",
    str::from_utf8(&output.stderr).unwrap()
  );

  str::from_utf8(&output.stdout).unwrap().trim().to_owned()
}

fn link(changelog: &str, mut author: impl FnMut(u64) -> String) -> String {
  Regex::new(r"\(#([0-9]+)( by @[^\s)]+)?\)")
    .unwrap()
    .replace_all(changelog, |captures: &Captures| {
      let pr = captures[1].parse::<u64>().unwrap();
      let contributor = author(pr);

      format!("([#{pr}](https://github.com/{REPO}/pull/{pr}) by [{contributor}](https://github.com/{contributor}))")
    })
    .into_owned()
}

fn main() {
  fs::write(
    "CHANGELOG.md",
    link(&fs::read_to_string("CHANGELOG.md").unwrap(), author),
  )
  .unwrap();
}
