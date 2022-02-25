default:
  just --list

all: build clippy fmt-check forbid readme test

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

run *args:
	cargo run -- --{{args}}

readme:
	cargo run -- --in-place --path README.md

test:
	cargo test

usage:
	cargo run -- --help | pbcopy

watch +COMMAND='test':
	cargo watch --clear --exec "{{COMMAND}}"
