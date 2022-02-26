use {
  executable_path::executable_path,
  pretty_assertions::assert_eq,
  std::{fs, process::Command, str},
  tempdir::TempDir,
  unindent::Unindent,
};

type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

struct Test {
  arguments: Vec<String>,
  expected_status: i32,
  expected_stderr: String,
  expected_stdout: String,
  markdown: String,
  tempdir: TempDir,
}

impl Test {
  fn new() -> Result<Self> {
    Ok(Self {
      arguments: Vec::new(),
      expected_status: 0,
      markdown: String::new(),
      expected_stderr: String::new(),
      expected_stdout: String::new(),
      tempdir: TempDir::new("test")?,
    })
  }

  fn expected_status(self, expected_status: i32) -> Self {
    Self {
      expected_status,
      ..self
    }
  }

  fn markdown(self, markdown: &str) -> Self {
    Self {
      markdown: markdown.unindent(),
      ..self
    }
  }

  fn expected_stdout(self, expected_stdout: &str) -> Self {
    Self {
      expected_stdout: expected_stdout.unindent(),
      ..self
    }
  }

  fn expected_stderr(self, expected_stderr: &str) -> Self {
    Self {
      expected_stderr: expected_stderr.unindent(),
      ..self
    }
  }

  fn argument(mut self, argument: &str) -> Self {
    self.arguments.push(argument.to_owned());
    self
  }

  fn run(self) -> Result {
    self.run_and_return_tempdir().map(|_| ())
  }

  fn command(&self) -> Result<Command> {
    let mut command = Command::new(executable_path(env!("CARGO_PKG_NAME")));

    fs::write(self.tempdir.path().join("foo.md"), self.markdown.clone())?;

    command
      .current_dir(&self.tempdir)
      .arg("--path")
      .arg(self.tempdir.path().join("foo.md"))
      .args(self.arguments.clone());

    Ok(command)
  }

  fn run_and_return_tempdir(self) -> Result<TempDir> {
    let output = self.command()?.output()?;

    let stderr = str::from_utf8(&output.stderr)?;

    assert_eq!(
      output.status.code(),
      Some(self.expected_status),
      "\n\nMarkdown:\n\n{}\nfailed: {}",
      self.markdown,
      stderr
    );

    if self.expected_stderr.is_empty() && !stderr.is_empty() {
      panic!("Expected empty stderr: {}", stderr);
    } else {
      assert_eq!(stderr, self.expected_stderr);
    }

    assert_eq!(str::from_utf8(&output.stdout)?, self.expected_stdout);

    Ok(self.tempdir)
  }
}

#[test]
fn simple() -> Result {
  Test::new()?
    .markdown(
      "
      ```present echo foo
      ```
      ",
    )
    .expected_status(0)
    .expected_stdout(
      "
      ```present echo foo
      foo
      ```
      ",
    )
    .run()
}

#[ignore]
#[test]
fn without_newline() -> Result {
  Test::new()?
    .markdown(
      "
      ```present echo foo```
      ",
    )
    .expected_status(0)
    .expected_stdout(
      "
      ```present echo foo
      foo
      ```
      ",
    )
    .run()
}

#[test]
fn codeblock_end_with_superfluous_characters() -> Result {
  Test::new()?
    .markdown(
      "
      ```present echo foo
      ```test
      ",
    )
    .expected_status(0)
    .expected_stdout(
      "
      ```present echo foo
      foo
      ```test
      ",
    )
    .run()
}

#[test]
fn invalid_command() -> Result {
  Test::new()?
    .markdown(
      "
      ```present foobarbaz
      ```test
      ",
    )
    .expected_status(1)
    .expected_stderr(
      "
      error: IO Error: No such file or directory (os error 2)
      ",
    )
    .run()
}

#[test]
fn simple_with_exterior_content() -> Result {
  Test::new()?
    .markdown(
      "
      foo!

      ```present echo bar
      ```

      baz!
      ",
    )
    .expected_status(0)
    .expected_stdout(
      "
      foo!

      ```present echo bar
      bar
      ```

      baz!
      ",
    )
    .run()
}

#[test]
fn multiple_commands() -> Result {
  Test::new()?
    .markdown(
      "
      ```present echo foo
      ```

      ```present echo bar
      ```

      ```present echo baz
      ```
      ",
    )
    .expected_status(0)
    .expected_stdout(
      "
      ```present echo foo
      foo
      ```

      ```present echo bar
      bar
      ```

      ```present echo baz
      baz
      ```
      ",
    )
    .run()
}

#[test]
fn multiple_commands_with_exterior_content() -> Result {
  Test::new()?
    .markdown(
      "
      one

      ```present echo foo
      ```

      two

      ```present echo bar
      ```

      three

      ```present echo baz
      ```
      ",
    )
    .expected_status(0)
    .expected_stdout(
      "
      one

      ```present echo foo
      foo
      ```

      two

      ```present echo bar
      bar
      ```

      three

      ```present echo baz
      baz
      ```
      ",
    )
    .run()
}

#[test]
fn remove_command() -> Result {
  Test::new()?
    .markdown(
      "
      ```present echo foo
      ```
      ",
    )
    .argument("--remove")
    .expected_status(0)
    .expected_stdout(
      "
      foo
      ",
    )
    .run()
}

#[test]
fn remove_command_with_exterior_content() -> Result {
  Test::new()?
    .markdown(
      "
      one

      ```present echo foo
      ```
      ",
    )
    .argument("--remove")
    .expected_status(0)
    .expected_stdout(
      "
      one

      foo
      ",
    )
    .run()
}

#[test]
fn remove_multiple_commands() -> Result {
  Test::new()?
    .markdown(
      "
      ```present echo foo
      ```

      ```present echo bar
      ```

      ```present echo baz
      ```
      ",
    )
    .argument("--remove")
    .expected_status(0)
    .expected_stdout(
      "
      foo

      bar

      baz
      ",
    )
    .run()
}

#[test]
fn remove_multiple_commands_with_exterior_content() -> Result {
  Test::new()?
    .markdown(
      "
      one

      ```present echo foo
      ```

      two

      ```present echo bar
      ```

      three

      ```present echo baz
      ```
      ",
    )
    .argument("--remove")
    .expected_status(0)
    .expected_stdout(
      "
      one

      foo

      two

      bar

      three

      baz
      ",
    )
    .run()
}

#[test]
fn codeblock_with_content() -> Result {
  Test::new()?
    .markdown(
      "
      ```present echo foo

      bar
      baz

      ```
      ",
    )
    .expected_status(0)
    .expected_stdout(
      "
      ```present echo foo
      foo
      ```
      ",
    )
    .run()
}
