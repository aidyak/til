# TIL (Today I Learned)

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
til ~/til
```

- `~/til/YYYY-MM-DD-til.md` if not existed
- Open file via Neovim

### Open file directly
```bash
til ~/til --file
```

### Search markdown contents
```bash
til ~/til --grep Rust
```

- Search inside `*.md` files with `ripgrep`

### Search markdown file names
```bash
til ~/til --files rust
```

- Search `*.md` file paths with `ripgrep`
