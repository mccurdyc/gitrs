<!-- vale off -->
# gitrs 🗂️

A simple, opinionated, tool, written in Rust, for declaretively managing Git
repos on your machine.
<!-- vale on -->

## Usage

Global arguments

- `--root <path>` - specify `$GITRS_ROOT`. Defaults to `$HOME/src`.

Subcommands

- `add <url>` - adds repo to the config file.
- `remove <url>` - remove repo from the config file.
- `sync` - reads the config file and adds or removes repos from the filesystem
to match the state of the config.

## Design goals

- Do one thing well: clone, update or remove repos from the filesystem.
  - Won't support running commands against cloned repos.
- Opinionated file structure. For example, `$GOPATH`. But you can specify a `GITRS_ROOT`.
- Have a single, config file for declaring the repos to manage.
- The config file is the source of truth for all repos cloned to your machine.
  - If you run `add`, it adds the repo to the config file.
  - If you run `remove`, it removes the repo from the config file.
  - Then, `sync` clones the repo and/or updates the filesystem to reflect
  the state of the config file.
- Changes to the config file could trigger a command to run.
  - Maybe a `watch` or something.
- Config file (thinking YAML)
  - Supports comments
  - If you don't want to use `gitrs` you can parse for use in another tool
  - A source of truth for repos your team (at work) needs

## TODO

- [ ] (TODO) lockfile / statefile.
- [ ] (TODO) pinning / skipping a repo from being checked for updates.
- [ ] (TODO) `sync --clean` - only remove repositories, doesn't update or clone.
- [ ] (TODO) `sync --archive` - archives repositories, to `$GITRS_ROOT/.archived`.
- [ ] (TODO) `watch` - watches the config file for updates and syncs the filesystem.
- [ ] (TODO) `list` - lists repos in the config file.
- [ ] (TODO) `status` - checks to see if cloned repos, need removed and/or if
  remote updates need fetched.
- [ ] (TODO) Nix package

## Inspiration

See [similar projects](./docs/inspiration.md).
