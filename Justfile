#!/usr/bin/env -S just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.

log := "warn"
export JUST_LOG := log

build:
    cargo build --release --bin app

test:
    cargo test -- --nocapture

lint:
    cargo clippy
