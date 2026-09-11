default:
  just --list

all: build test clippy fmt-check forbid readme

alias f := fmt
alias r := run

[group: 'misc']
build:
  cargo build --workspace

[group: 'release']
changelog:
  cargo run --package present-changelog

[group: 'check']
check:
 cargo check --workspace

[group: 'check']
clippy:
  cargo clippy --workspace --all-targets --all-features

[group: 'format']
fmt:
  cargo +nightly fmt --all

[group: 'check']
fmt-check:
  cargo +nightly fmt --all -- --check
  @echo formatting check done

[group: 'check']
forbid:
  ./bin/forbid

[group: 'release']
publish:
  #!/usr/bin/env bash
  set -euxo pipefail
  rm -rf tmp/release
  gh repo clone https://github.com/terror/present tmp/release
  cd tmp/release
  VERSION=`sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`
  git tag -a $VERSION -m "Release $VERSION"
  git push origin $VERSION
  cargo publish
  cd ../..
  rm -rf tmp/release

[group: 'dev']
run *args:
  cargo run -- {{args}}

[group: 'misc']
readme:
  cargo run -- README.md --in-place

[group: 'test']
test *args:
  cargo test --workspace --all-targets {{args}}

[group: 'test']
test-release-workflow:
  -git tag -d test-release
  -git push origin :test-release
  git tag test-release
  git push origin test-release

[group: 'release']
update-changelog *args:
  echo >> CHANGELOG.md
  git log --first-parent --pretty='format:- %s' {{args}} >> CHANGELOG.md

[group: 'dev']
watch +COMMAND='test':
  cargo watch --clear --exec "{{COMMAND}}"
