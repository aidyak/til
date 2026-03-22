# TIL (Today I Learned)

[![CI](https://github.com/aidyak/til/actions/workflows/ci.yml/badge.svg)](https://github.com/aidyak/til/actions/workflows/ci.yml)

CLI for management your TIL markdown.
You can open your md file at your directory and create simple md file.

## How to use

### Installation

```bash
git clone https://github.com/aidyak/til
cd til
cargo install --path .
```

### Basic Usage

```bash
til setup ~/til
til
```

- `~/til/YYYY-MM-DD-til.md` if not existed
- Open directory via Neovim
- `til setup <dir>` saves the base directory for later runs

### Open another directory once
```bash
til ~/other-til
```

- Temporarily overrides the saved base directory
- Does not change the configured base directory

### Open file directly
```bash
til --file
```

### Search markdown contents
```bash
til --grep Rust
```

- Search inside `*.md` files with `ripgrep`

### Search markdown file names
```bash
til --files rust
```

- Search `*.md` file paths with `ripgrep`
