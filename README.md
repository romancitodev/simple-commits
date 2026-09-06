<!-- markdownlint-disable no-inline-html first-line-heading -->

<div align="center">

# `✨ simple-commits`

**A small CLI that turns your dirty commits into conventional ones**

[![Check CI](https://github.com/romancitodev/simple-commits/actions/workflows/checks.yml/badge.svg?branch=main)](https://github.com/romancitodev/simple-commits/actions/workflows/checks.yml)
[![Crates.io](https://img.shields.io/crates/v/simple-commit.svg)](https://crates.io/crates/simple-commit)
[![Downloads](https://img.shields.io/crates/d/simple-commit.svg)](https://crates.io/crates/simple-commit)
[![License](https://img.shields.io/crates/l/simple-commit.svg)](LICENSE)

</div>

## About

`sc` walks you through building a [Conventional Commit](https://www.conventionalcommits.org/) message step by step — type, scope, gitmoji, title, body, footer, and breaking changes — then commits straight through `git2`: GPG signing, your repo's hooks, and reusing whatever your `gpg-agent` already has cached, all included.

## ✨ Features

- Fully conventional commits, gitmoji included
- Native git access via `git2`, no shell `git` required for the core flow
- GPG-signed commits, retried in place on a bad passphrase, reusing the agent's cache when it's warm
- Runs your repo's `pre-commit`, `commit-msg`, and `post-commit` hooks
- Infers commit type, scope, and a linked issue (`Closes: #123`) from your current branch name
- Custom scopes with fuzzy autocomplete, saved back to your config as you use them
- Custom commit templates, e.g. auto `git push` right after committing

## 📥 Installation

```bash
cargo install simple-commit
```

NixOS:

```bash
nix profile install github:romancitodev/simple-commits
```

## 🛠 Configuration

In your `~/$CONFIG_FOLDER` create an `sc` directory with a `config.toml` inside:

> [!TIP]
>
> ```bash
> mkdir ~/$CONFIG_FOLDER/sc && touch ~/$CONFIG_FOLDER/sc/config.toml
> ```

Then use this template as a starting point:

```toml
# The scopes
[[scopes]]
name = "app"
description = "the app itself"

[[scopes]]
name = "core"
description = "the core lib"

[git]
skip_emoji = true

# Customize your commit template as you want
commit_template = ["git", "commit", "-m", "{{message}}", "&&", "git", "push"]
```

## 💻 Usage

Just run it:

```bash
sc
```

Or with flags:

| flag                        | Description                                   |
| ---------------------------- | ---------------------------------------------- |
| `-e` \| `--skip-emoji`       | Skips the emoji step                           |
| `-c` \| `--commit-template`  | Command to run after generating the commit     |
| `--config`                   | Set the config path                            |

## 🤝 Contributing

Issues and pull requests are both very welcome at [romancitodev/simple-commits](https://github.com/romancitodev/simple-commits).

## 📄 License

Licensed under the Apache License, Version 2.0 ([LICENSE](LICENSE) or <http://www.apache.org/licenses/LICENSE-2.0>).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be licensed as above, without any additional terms or conditions.
