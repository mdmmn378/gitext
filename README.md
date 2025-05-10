# gitext

A command-line tool that converts a Git repository into a single text file optimized for LLM consumption.

> 🚀 This project was fully developed with AI assistance using [Cursor](https://cursor.sh)

## Author

[Mamun](https://github.com/mdmmn378)

## Features

- Extracts source code and documentation from Git repositories
- Supports multiple file types (Rust, Python, Go, TypeScript, JavaScript, Java, C++, C, Markdown, etc.)
- Respects .gitignore by default
- Configurable output file
- Simple and fast

## Installation

### From Binary Release

Download the latest release from the [Releases](https://github.com/mdmmn378/gitext/releases/latest) page:

- **Linux**: Download `gitext-linux-x86_64.tar.gz`

  ```bash
  tar xzf gitext-linux-x86_64.tar.gz
  sudo mv gitext /usr/local/bin/
  ```

- **macOS**: Download `gitext-macos-x86_64.tar.gz`

  ```bash
  tar xzf gitext-macos-x86_64.tar.gz
  sudo mv gitext /usr/local/bin/
  ```

- **Windows**: Download `gitext-windows-x86_64.exe.zip`
  - Extract the zip file
  - Move `gitext.exe` to a directory in your PATH

### From Source

```bash
cargo install --path .
```

## Usage

```bash
gitext --repo /path/to/repo --output summary.txt
```

### Options

- `-r, --repo`: Path to Git repository (required)
- `-o, --output`: Output file path (default: output.txt)
- `--no-gitignore`: Include files ignored by .gitignore

## Example

```bash
gitext -r ./my-project -o project-summary.txt
```

## License

MIT
