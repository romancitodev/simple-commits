# Simple commits

A little CLI written in rust to improve your dirty commits into **conventional** ones.
## 👀 Demo _(coming soon)_


## ✨ Features

- Fully conventional commits
- Auto-commit
- Custom templates
- Written in rust


## 📥 Installation _(not available yet)_

Install it using cargo!

```bash
cargo install simple-commits
```

    
## 🛠 Configuration

in your `~/$CONFIG_FOLDER` create a `sc` directory with a `config.toml` inside.

>   [!TIP]
>   ```bash
>   mkdir ~/$CONFIG_FOLDER/sc && touch ~/$CONFIG_FOLDER/sc/config.toml
>   ```

and use this template to configure it as you want.

```toml
# The scopes
scopes = ["app", "lib", "docs"]

[git]
# By default the skip preview flag is setted to false because we know
# It's a dangerous action.
skip_preview = true
# Customize your commit template as you want
commit_template = ["git", "commit", "-m", "{{message}}", "&&", "git", "push"]
```
## 💻 Usage

To use it you just need to run one command. 😍

```bash
sc
```

or if you prefer to want to use flags:

| flags | Description |
| ----- | ----------- |
| `-s` \| `--skip-preview` | ⚠️ Skips the preview step (Dangerous) |
| `-c` \| `--commit-template` | Command to run after generate commit message |
| `--config` | Set the config path |
