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
  #!/usr/bin/env bash
  set -euxo pipefail
  rm -rf tmp/release
  git clone https://github.com/terror/present.git tmp/release
  VERSION=`sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`
  cd tmp/release
  git tag -a $VERSION -m "Release $VERSION"
  git push origin $VERSION
  cargo publish
  cd ../..
  rm -rf tmp/release

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
