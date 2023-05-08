# gitrs 

A simple, opinionated, tool, written in Rust, for declaratively managing Git
repos on your machine.

## Usage

- `add` - adds repo to the config file.
- `remove` - remove repo from the config file.
  - default is to hard remove the directory from the filesystem
  - can specify `--archive` which moves a directory to `$GITRS_ROOT` instead of
  hard removing.
- `sync` - reads the config file and adds or removes repos from the filesystem
to match the state of the config.
- (TODO) `watch` - watches the config file for updates and syncs the filesystem.
- (TODO) `list` - lists repos in the config file.
- (TODO) `status` - checks to see if repos are cloned, need remove and if there are
remote updates to fetch.

## Design Goal(s)

- Do one thing well: clone, update or remove repos from the filesystem.
  - Won't support running commands against cloned repos.
- Opinionated file structure e.g., `$GOPATH`. But you can specify a `GITRS_ROOT`.
- Have a single, config file for declaring the repos to manage.
- The config file should be the source of truth for all repos cloned to your machine.
  - If you run `add`, it adds the repo to the config file.
  - If you run `remove`, it removes the repo from the config file.
  - Then, `sync` clones the repo and/or updates the filesystem to reflect
  the state of the config file.
- Changes to the config file could trigger a command to be run.
  - Maybe a `watch` or something.
- Config file (thinking YAML)
  - Supports comments
  - If you didn't want to use `gitrs` could easily be parsed for use in another tool
    - i.e., the config file interface should be easy to parse / understand
  - Could be a source of truth for repos your team (at work) needs

## TODOs

- [ ] nix package
- [ ] (TODO) `watch` - watches the config file for updates and syncs the filesystem.
- [ ] (TODO) `list` - lists repos in the config file.
- [ ] (TODO) `status` - checks to see if repos are cloned, need remove and if
  there are remote updates to fetch.

## Inspiration

See [similar projects](./docs/inspiration.md).
