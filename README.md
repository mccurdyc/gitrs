<!-- vale off -->
# gitrs 🗂️

A simple, opinionated, tool, written in Rust, for declaretively managing Git
repos on your machine.
<!-- vale on -->

"simple" - limited in what it supports. For example, won't support running commands
against repos.
"opinionated" - similar to Go and the old `$GOPATH` is how repos are stored.

## Usage

Global arguments

- `--root <path>` - specify `$GITRS_ROOT`. Defaults to `$HOME/src`.

Subcommands

- `add <url>` - adds repo to the config file.
- `remove <url>` - remove repo from the config file.
- `sync` - reads the config file and adds or removes repos from the filesystem
to match the state of the config.

## `$GITRS_ROOT/.gitrs.yaml` config file

```yaml
metadata:
 version: v1beta
 root: /home/user/src
 last_sync: <timestamp>
repos:
- name: github.com/mccurdyc/gitrs
  pin: <true|default:false>
  sha: <sha>
```

## Design goals

- Do one thing well: clone, update or remove repos from the filesystem.
  - Won't support running commands against cloned repos.
- Only supports SSH cloning, [similar to Go](https://cs.opensource.google/go/go/+/refs/heads/master:src/cmd/go/internal/get/get.go%3Bdrc=91b8cc0dfaae12af1a89e2b7ad3da10728883ee1%3Bl=423).
- Opinionated file structure. For example, `$GOPATH`. But you can specify a `GITRS_ROOT`.
- You could have multiple "roots" for different uses.
For example, `$HOME/{work,personal}` with separate gitrs configs.
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
- [ ] (TODO) `add --pin [<SHA>]` pinning / skipping a repo from being checked for updates.
- [ ] (TODO) `sync --restore <FILE>` - restore from a gitrs lockfile.
- [ ] (TODO) `sync --clean` - only remove repositories, doesn't update or clone.
- [ ] (CONSIDER) `sync --archive` - archives repositories, to `$GITRS_ROOT/.archived`.
- [ ] (TODO) `watch` - watches the config file for updates and syncs the filesystem.
- [ ] (TODO) `list` - lists repos in the config file.
- [ ] (TODO) `status` - checks to see if cloned repos, need removed and/or if
  remote updates need fetched.
- [ ] (TODO) Nix package

## Adoption

- I'm still considering whether or not I want to `add` to support multiple repos.

```bash
mv src/ src.bak/

for d in ~/src.bak/github.com/org/*; do
  gitrs add $(echo ${d##*src.bak/})
done

gitrs sync
```

## Inspiration

See [similar projects](./docs/inspiration.md).

## LICENSE

See [LICENSE.md](./LICENSE.md).
