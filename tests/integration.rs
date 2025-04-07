use {
  executable_path::executable_path,
  pretty_assertions::assert_eq,
  std::{
    fs,
    io::Write,
    process::{Command, Stdio},
    str,
  },
  tempdir::TempDir,
  unindent::Unindent,
};

type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

struct Test {
  arguments: Vec<String>,
  expected_status: i32,
  expected_stderr: String,
  expected_stdout: String,
  markdown: Vec<String>,
  tempdir: TempDir,
}

impl Test {
  fn new() -> Result<Self> {
    Ok(Self {
      arguments: Vec::new(),
      expected_status: 0,
      expected_stderr: String::new(),
      expected_stdout: String::new(),
      markdown: Vec::new(),
      tempdir: TempDir::new("test")?,
    })
  }

  fn argument(mut self, argument: &str) -> Self {
    self.arguments.push(argument.to_owned());
    self
  }

  fn expected_status(self, expected_status: i32) -> Self {
    Self {
      expected_status,
      ..self
    }
  }

  fn expected_stderr(self, expected_stderr: &str) -> Self {
    Self {
      expected_stderr: expected_stderr.unindent(),
      ..self
    }
  }

  fn expected_stdout(self, expected_stdout: &str) -> Self {
    Self {
      expected_stdout: expected_stdout.unindent(),
      ..self
    }
  }

  fn markdown(mut self, markdown: &str) -> Self {
    self.markdown.push(markdown.unindent());
    self
  }

  fn run(self) -> Result {
    self.run_and_return_tempdir().map(|_| ())
  }

  fn command(&self) -> Result<Command> {
    let mut command = Command::new(executable_path(env!("CARGO_PKG_NAME")));

    self
      .markdown
      .iter()
      .enumerate()
      .try_for_each(|(index, markdown)| {
        fs::write(
          self.tempdir.path().join(format!("test-{}.md", index)),
          markdown,
        )
      })?;

    command
      .current_dir(&self.tempdir)
      .arg(self.tempdir.path())
      .args(self.arguments.clone());

    Ok(command)
  }

  fn tempdir(self) -> Result<TempDir> {
    self.command()?.output()?;
    Ok(self.tempdir)
  }

  fn run_and_return_tempdir(self) -> Result<TempDir> {
    let output = self.command()?.output()?;

    let stderr = str::from_utf8(&output.stderr)?;

    assert_eq!(
      output.status.code(),
      Some(self.expected_status),
      "\n\nMarkdown:\n{:?}\nfailed with output: {}",
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

#[test]
fn arbitrary_fence_length() -> Result {
  Test::new()?
    .markdown(
      "
      `````present echo foo
      `````
      ",
    )
    .expected_status(0)
    .expected_stdout(
      "
      `````present echo foo
      foo
      `````
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
#[cfg(not(target_os = "windows"))]
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
      error: Program foobarbaz failed to execute with message: No such file or directory (os error 2)
      ",
    )
    .run()
}

#[test]
#[cfg(target_os = "windows")]
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
      error: Program foobarbaz failed to execute with message: program not found
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

#[test]
fn multiple_markdown_files() -> Result {
  Test::new()?
    .markdown(
      "
      ```present echo foo
      ```
      ",
    )
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
      ```present echo foo
      foo
      ```
      ",
    )
    .run()
}

#[test]
fn inline_script_simple() -> Result {
  Test::new()?
    .markdown(
      "
      ```present bash -c 'echo foo'
      ```
      ",
    )
    .expected_status(0)
    .expected_stdout(
      "
      ```present bash -c 'echo foo'
      foo
      ```
      ",
    )
    .run()
}

#[test]
fn inline_script_complex() -> Result {
  Test::new()?
    .markdown(
      "
      ```present bash -c 'for i in {1..10}; do echo $i; done'
      ```
      ",
    )
    .expected_status(0)
    .expected_stdout(
      "
      ```present bash -c 'for i in {1..10}; do echo $i; done'
      1
      2
      3
      4
      5
      6
      7
      8
      9
      10
      ```
      ",
    )
    .run()
}

#[test]
fn inline_unmatched_delimiter() -> Result {
  Test::new()?
    .markdown(
      "
      ```present bash -c 'echo foo
      ```
      ",
    )
    .expected_status(1)
    .expected_stderr(
      "
      error: Lex Error: Unmatched delimiter
      ",
    )
    .run()
}

#[test]
fn interactive_accept() -> Result {
  let tempdir = Test::new()?
    .markdown("```present echo foo\n```")
    .tempdir()?;

  let mut command = Command::new(executable_path(env!("CARGO_PKG_NAME")))
    .args([tempdir.path().to_str().unwrap(), "--interactive"])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;

  write!(command.stdin.as_mut().unwrap(), "y")?;

  assert_eq!(
    str::from_utf8(&command.wait_with_output()?.stdout)?,
    "```present echo foo\nfoo\n```"
  );

  Ok(())
}

#[test]
fn interactive_reject() -> Result {
  let tempdir = Test::new()?
    .markdown("```present echo foo\n```")
    .tempdir()?;

  let mut command = Command::new(executable_path(env!("CARGO_PKG_NAME")))
    .args([tempdir.path().to_str().unwrap(), "--interactive"])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;

  write!(command.stdin.as_mut().unwrap(), "n")?;

  assert_eq!(
    str::from_utf8(&command.wait_with_output()?.stdout)?,
    "```present echo foo\n```"
  );

  Ok(())
}

#[test]
fn grapheme_handling() -> Result {
  Test::new()?
    .markdown(
      r#"
      Hello, 世界! 👋

      ```present echo "🚀 Grapheme test: é, 世界, 👨‍👩‍👧‍👦"
      ```

      Grapheme cluster: 👨‍👩‍👧‍👦
      "#,
    )
    .expected_status(0)
    .expected_stdout(
      r#"
      Hello, 世界! 👋

      ```present echo "🚀 Grapheme test: é, 世界, 👨‍👩‍👧‍👦"
      🚀 Grapheme test: é, 世界, 👨‍👩‍👧‍👦
      ```

      Grapheme cluster: 👨‍👩‍👧‍👦
      "#,
    )
    .run()
}

#[test]
#[cfg(not(target_os = "windows"))]
fn large_output_handling() -> Result {
  Test::new()?
    .markdown(
      r#"
      ```present python -c "print('Large ' * 1000)"
      ```
      "#,
    )
    .expected_status(0)
    .expected_stdout(&format!(
      r#"
      ```present python -c "print('Large ' * 1000)"
      {}
      ```
      "#,
      "Large ".repeat(1000)
    ))
    .run()
}

#[test]
fn escaping_special_characters() -> Result {
  Test::new()?
    .markdown(
      r#"
      ```present echo "Special chars: && || > < | ; \" ' \\"
      ```
      "#,
    )
    .expected_status(0)
    .expected_stdout(
      r#"
      ```present echo "Special chars: && || > < | ; \" ' \\"
      Special chars: && || > < | ; " ' \
      ```
      "#,
    )
    .run()
}

#[test]
fn complex_shell_pipeline() -> Result {
  Test::new()?
  .markdown(
    r#"
    ```present bash -c "echo 'hello world' | tr ' ' '\n' | sort | uniq -c | sort -nr | sed 's/^[[:space:]]*//' "
    ```
    "#,
  )
  .expected_status(0)
  .expected_stdout(
    r#"
    ```present bash -c "echo 'hello world' | tr ' ' '\n' | sort | uniq -c | sort -nr | sed 's/^[[:space:]]*//' "
    1 world
    1 hello
    ```
    "#,
  )
  .run()
}

#[test]
fn unicode_normalization() -> Result {
  Test::new()?
    .markdown(
      r#"
      ```present bash -c "echo \"é\" | xxd -p && echo \"é\" | xxd -p"
      ```
      "#,
    )
    .expected_status(0)
    .expected_stdout(
      r#"
      ```present bash -c "echo \"é\" | xxd -p && echo \"é\" | xxd -p"
      c3a90a
      65cc810a
      ```
      "#,
    )
    .run()
}
