<h1 align="center">mq-check</h1>

A syntax and semantic checker for [mq](https://github.com/harehare/mq) files. Validates `.mq` query files for errors and warnings, providing colored diagnostic output with precise line and column information.

![demo](./assets/demo.gif)

## Features

- Detects syntax errors and semantic issues in mq files
- Colored terminal output (red for errors, yellow for warnings)
- Precise error locations with line and column numbers
- Supports checking multiple files at once
- Suitable for CI/CD pipelines (non-zero exit code on errors)
- Available as both a CLI tool and a library

## Installation

### From source

### Using the Installation Script (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/harehare/mq-check/main/bin/install.sh | bash
```

The installer will:
- Download the latest release for your platform
- Verify the binary with SHA256 checksum
- Install to `~/.mq-check/bin/`
- Update your shell profile (bash, zsh, or fish)

After installation, restart your terminal or run:
```bash
source ~/.bashrc  # or ~/.zshrc, or ~/.config/fish/config.fish
```

### Cargo

```bash
# Install from crates.io
cargo install mq-check
# Install using binstall
cargo binstall mq-check@0.1.0
```

### From Source

```bash
git clone https://github.com/harehare/mq-check.git
cd mq-check
cargo build --release
# Binary will be at target/release/mq-check
```

## Usage

### CLI

```bash
# Check a single file
mq-check query.mq

# Check multiple files
mq-check query1.mq query2.mq

# Use in CI/CD
mq-check *.mq && echo "All checks passed"
```

#### Example output

```
Checking: query.mq
  Error: Expected `;` but found `|` at line 2, column 5
  Warning: Function `old_func` is deprecated at line 3, column 10
```

### Library

```rust
use std::path::PathBuf;

fn main() -> miette::Result<()> {
    let files = vec![PathBuf::from("query.mq")];
    mq_check::check_files(&files)
}
```

## License

MIT
