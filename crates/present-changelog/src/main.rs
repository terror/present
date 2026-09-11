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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn linking() {
    #[track_caller]
    fn case(changelog: &str, expected: &str) {
      assert_eq!(link(changelog, |pr| format!("foo{pr}")), expected);
    }

    case("", "");
    case("foo", "foo");
    case("foo (#bar)", "foo (#bar)");

    let linked = "foo ([#1](https://github.com/terror/present/pull/1) by [foo1](https://github.com/foo1))";

    for changelog in [
      "foo (#1)",
      "foo (#1 by @bar)",
      "foo (#1 by @Bar-baz)",
      "foo (#1 by @bar[bot])",
      linked,
    ] {
      case(changelog, linked);
    }

    case(
      "foo (#1)\nbar (#2)",
      "foo ([#1](https://github.com/terror/present/pull/1) by [foo1](https://github.com/foo1))\nbar ([#2](https://github.com/terror/present/pull/2) by [foo2](https://github.com/foo2))",
    );
  }
}
