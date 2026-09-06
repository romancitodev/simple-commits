
## [unreleased]

### ⚙️ Miscellaneous Tasks

- Add Dependabot for cargo and github-actions

### Build

- *(deps)* Bump the cargo-dependencies group with 4 updates
- *(deps)* Bump the github-actions group with 2 updates

## New Version [2.0.0-rc.1]

### 🚀 Features

- *(tui)* ✨ retry a bad GPG passphrase in place instead of stacking new steps
- *(tui)* ✨ turn scope picker into free-text autocomplete
- ✨ add git2-backed git module with GPG signing and hooks
- *(cli)* ✨ Add styled prompts, visible feedback and file staging
- *(cli)* [**breaking**] ✨ update finally the multiselect api
- 🚧 working on the refactor
- *(cli)* ✨ add new flag `--message` to automatize pipelines
- *(core)* 🔧 new subcommands (init)

### 🐛 Bug Fixes

- Add authors and homepage to Cargo.toml
- *(ci)* Bump cargo-dist to 0.32.0 and regenerate the release workflow
- *(ci)* Use ubuntu-latest instead of the retired ubuntu-20.04 runner
- *(git)* 🐛 wrong gpg-agent-connect
- *(tui)* 🚑️ gpg cache signing
- *(tui)* 🐛 stop swallowing commit/staging errors, retry on bad GPG passphrase
- *(tui)* 🐛 resolve gpg-agent cache correctly and surface a bad-passphrase error
- *(cli)* 🐛 exclamation symbol on non-breaking-change commits
- Simplify Rust toolchain setup to avoid manual hash updates
- Refactor apps configuration to explicitly define program path and type
- 🔧 load local config files
- Switch sha256
- *(core)* Removed & on function call

### 🚜 Refactor

- *(tui)* ♻️ simplified code
- *(cli)* Migrating components to `nobubbles`
- *(cli)* [**breaking**] 🏗️ Rename from `gen` to `gitmoji`
- *(core)* 🔥 Drop the old widgets that weren't used tho
- *(core)* TUI steps to use pipeline pattern.
- *(cli)* 🚨 fix lint warnings
- *(cli)* ➕ change promptuity for cliclack
- *(core)* 🏷️ rename State for AppData for future changes
- *(core)* 🏗️ accommodating files
- *(core)* 🏗️ internal state and  commit builder

### 📚 Documentation

- 🍱 testing some stuff
- 📝 updated readme
- NixOS install instructions for simple-commits
- Update README.md

### 🎨 Styling

- Collapse a nested if per clippy

### ⚙️ Miscellaneous Tasks

- Release simple-commit version 2.0.0-rc.1
- 🎨 run cargo fmt --all
- 🎨 formatted all the code

### Build

- 🏗️ Switch to `nobubbles` and bump edition to 2024
- ⬆️ bump dependencies

## New Version [1.0.2]

### 🐛 Bug Fixes

- 🐛 fuzzy finder behaviour
- 🐛 fix the skip_preview config

### ⚙️ Miscellaneous Tasks

- Release simple-commit version 1.0.2
- Update ci

## New Version [1.0.1]

### 🚀 Features

- 💄 refactored totally UI and functionality

### 🚜 Refactor

- ⚡️ change inline assigment instead of cloning

### ⚡ Performance

- 🏷️ replace &String for &str

### ⚙️ Miscellaneous Tasks

- Release simple-commit version 1.0.1
- Automatic generate changelog
- Update to new cargo-dist

## New Version [0.2.0]

### 'chore

- *(app)* Add from state for commits'
- *(app)* Add configs for git'

### 'feat

- *(app)* Add exec git commands'

### 🚀 Features

- :art: Formatted the readme and added the fields -p to --skip-preview and -e to --skip-emoji
- Finishing `scopes` steps and config
- Support for local, global and args configs
- Add emojis step
- :sparkles: better handling of errors
- Add custom validator for message step
- Add support for multiple steps & refactor
- Added steps architecture
- 🔨 add nix flake and rust toolchain file
- New proyect!

### 🐛 Bug Fixes

- The default names were left and the corresponding change was made to fix the error
- Fixed a naming bug and added emoji_skip in sc.toml
- 🐛 fixed empty scope
- :pencil2: typo on `skipped`
- :bug: fix builder returns
- *(app)* 🔥 remove unstable api

### 🚜 Refactor

- Doing the CLI stuff
- :fire: delete .idea folder
- :fire: remove unused code
- :recycle: removed unnecesary code
- :art: updated macro
- Moved files

### 📚 Documentation

- 📝 Update  README.md
- *(generation)* Add commentary for prevent manual modify

### 🎨 Styling

- Added more style to the options

### ⚙️ Miscellaneous Tasks

- Release simple-commit version 0.2.0
- *(app)* Add option to skip emojis step
- *(app)* 🚀 add release automatization
- *(app)* 🚀 init cargo dist to automate build and deploy release
- Add check ci workflow (#7)
- Rename simple commit config struct
- New format of config local
- :label: added sugar type

### Build

- 🏗️ add license mit to Cargo.toml
- *(app)* 🏗️ rename bin to sc and package to simple-commits
- *(tools)* Update nix rust toolchain hash
- Updated emojis from generation
- Updated emojis from generation
- Make automatic generation from pipeline github actions

### Core

- Set architecture


