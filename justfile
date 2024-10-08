default:
  just --list

all: build test clippy fmt-check forbid readme

alias f := fmt
alias r := run

build:
  cargo build

check:
 cargo check

clippy:
  cargo clippy --all-targets --all-features

fmt:
  cargo +nightly fmt

fmt-check:
  cargo +nightly fmt --all -- --check
  @echo formatting check done

forbid:
  ./bin/forbid

publish:
  ./bin/publish

run *args:
  cargo run -- {{args}}

readme:
  cargo run -- README.md --in-place

test *args:
  cargo test --all-targets {{args}}

usage:
  cargo run -- --help | pbcopy

watch +COMMAND='test':
  cargo watch --clear --exec "{{COMMAND}}"
