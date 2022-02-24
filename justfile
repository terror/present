default:
  just --list

all: build clippy fmt-check forbid test

alias f := fmt
alias r := run

build:
	cargo build

check:
 cargo check

test:
	cargo test

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

usage:
	cargo run -- --help | pbcopy

watch +COMMAND='test':
	cargo watch --clear --exec "{{COMMAND}}"
