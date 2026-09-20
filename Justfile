#!/usr/bin/env -S just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.

log := "warn"
export JUST_LOG := log

build:
    cargo build --release --bin gitrs

test:
    cargo test -- --nocapture

lint:
    cargo clippy

release: build
    #!/usr/bin/env bash
    set -euo pipefail

    version="v$(cargo pkgid | sed 's/.*[@#]//')"
    branch="$(git branch --show-current)"

    if [[ "$branch" != "main" ]]; then
        echo "Releases must be created from the main branch (currently on '$branch')" >&2
        exit 1
    fi

    if ! git diff --quiet; then
        echo "Working tree is dirty; commit or stash changes before releasing" >&2
        exit 1
    fi

    if git rev-parse "$version" >/dev/null 2>&1; then
        echo "Tag $version already exists" >&2
        exit 1
    fi

    git tag -a "$version" -m "Release $version"
    git push origin "$version"
    gh release create "$version" --generate-notes
